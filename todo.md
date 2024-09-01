This is a constantly evolving list of bugs and things to fix.
It also includes features that I want to add in the future.
# Bug Fixes
 - [x] Make things like ```test()[1].p``` actually work in the parser
 - [ ] Fix ```get_root_scope``` (Currently it doesn't work when evaluating within an object's scope)
 For this I might want to just predefine a hard global scope and use that instead of having a function for it.
 - [ ] Error message sometimes does not underline the whole item; basically just fix error syntax highlighting
 (this is not top priority)
# Questionable Fixes
Instead of solving the problem with parent scopes not being able to be accessed because of some
issue with Weak<>, I am just manually adding blueprints defined in the root scope to an object's
scope whenever the ```new``` operator is being used. This sort of works for now.
# Things To Add
 - [ ] Add more comments
 - [ ] Implement/improve GErrors for everything
 - [ ] Implement dot syntaxing default functions for strings, etc.
 - [ ] Add a graphics library
 - [ ] Implement warning system with config file to enable/disable