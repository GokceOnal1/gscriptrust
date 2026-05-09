This is a constantly evolving list of bugs and things to fix.
It also includes features that I want to add in the future.
# Bug Fixes
 - [ ] (low priority) Fix unnecessary divergent paths in ```obj_index_mut``` (see Claude conversation)
 - [ ] (medium priority) Fix issue where objects are deep cloned even when unnecessary (see Claude conversation)
# Questionable Fixes
Instead of solving the problem with parent scopes not being able to be accessed because of some
issue with ```Weak<>```, I am just manually adding blueprints defined in the root scope to an object's
scope whenever the ```new``` operator is being used. This sort of works for now.
# Things To Add
 - [ ] Add more comments
 - [ ] Implement/improve GErrors for everything
 - [ ] Implement dot syntaxing default functions for strings, etc.
 - [ ] Add a graphics library
 - [ ] Implement warning system with config file to enable/disable