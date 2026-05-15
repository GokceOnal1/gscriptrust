This is a constantly evolving list of bugs and things to fix.
It also includes features that I want to add in the future.
# Bug Fixes
 - [ ] (low priority) Fix unnecessary divergent paths in ```obj_index_mut``` (see Claude conversation)
 - [ ] (medium priority) Fix issue where objects are deep cloned even when unnecessary (see Claude conversation)
 - [ ] (medium priority) Need to come up with a way to instantiate imported blueprints
 - [ ] (high priority) Fix bug where you can't internally change self's properties from methods
# Questionable Fixes
Instead of solving the problem with parent scopes not being able to be accessed because of some
issue with ```Weak<>```, I am just manually adding blueprints defined in the root scope to an object's
scope whenever the ```new``` operator is being used. This sort of works for now.

Lot of old stuff and logic that I don't quite understand as I'm revisiting this project. Will be asking Claude to help
me understand and possibly refactor a lot of old things.
# Currently Working On
Right now, I am in the midst of wrapping strings and primitive types in blueprints (!!! WIP WIP WIP !!!)
# Things To Add
 - [ ] Add more comments
 - [ ] Implement/improve GErrors for everything
 - [ ] Continue implementing dot syntax for strings, primitives, and lists
 - [ ] Work on integrating an SDL3 wrapper
 - [ ] Implement warning system with config file to enable/disable
 - [ ] Implement passing objects by reference