# Small bugs

Bugs found while testing on 2026-10-15. Each has a minimal repro and a pointer to where the problem starts. None of them are related to the pass-by-value / pass-by-reference work.

## 1. `[]` is a one-element list

```
write([]);            \ prints [no operation] \
write([].length());   \ prints 1 \
```

`parse_list` (`src/parsing/parser.rs:378`) always parses a first element right after `[` before checking for `]`. For an empty list, `parse_comp_expr` returns `None`, which becomes a NOOP that gets pushed into `contents`.

## 2. `list[i].prop = value` silently ends the program

```
blueprint C { prop n; method create(param x) { n = x; }; };
assign px = [new C(1)];
px[0].n = 50;
write("never printed");
```

No output and no error. There are two causes:

- **The assignment is never parsed.** In `parse_variable` (`parser.rs:308-313`), the `var[i]` followed by `.` branch returns `parse_obj_index(...)` directly. Unlike the `var.x` branch just below it, it never checks for a following `=` and calls `parse_obj_reassign`.
- **The parser fails silently.** The leftover `=` reaches `parse_compound` (`parser.rs:39-48`), which stops at any token that isn't `;`. The rest of the file is dropped and no error is pushed, so any stray token anywhere in a file causes the same silent stop. An error when `parse_compound` stops on something that isn't the end of the input would have made this bug obvious.

## 3. String/list methods inside an imported file panic

```
\ entry/mod.gsc \
assign n = "abc".length();

\ entry/main.gsc \
import "mod.gsc" -> m;
```

This panics at `wrap_primitive` (`src/interpreting/visitor.rs`, the `resolve_blueprint(prim_name...).unwrap()`). `visit_import` runs the imported file in `Scope::new(None)`, a scope with no parent, so the preloaded `_string`/`_list` blueprints in the main root scope aren't reachable from it. Two parts to fix: decide how stdlib blueprints become visible to imported files, and replace the `unwrap` with a `GError`. This bug existed before `wrap_primitive`; `wrap_string` did the same thing.

## 4. Can't index into a list literal and then use a dot

```
write([[1,2],[3]][0].length());   \ TokenError: Expected token RPR but received DOT \
```

`assign l = [[1,2],[3]]; l[0].length();` works. The problem is only when the list is a literal. The literal's parse path handles `[...]` and `[...][i]` but never checks for a `.` after the index. Find where `parse_list` is called from `parse_mono` (`parser.rs:131`).

## 5. Non-boolean `while` condition exits silently

```
while ("x") { write("body"); };   \ loop is skipped, no error \
```

The crash is fixed, but the `None` branch in `visit_while` (`visitor.rs`, around line 919) just breaks. The comment there says this case should be an "invalid conditional expression error". `visit_if` (`visitor.rs:876-886`) already handles this case: it pushes `ETypes::ConditionalError` with "Expected conditional expression". Copy that into the `None` branch before the `break`.
