blueprint _string {
    prop _s;
    method create() {};
    method contains(param c) {
        assign i = 0;
        assign found = false;
        while(i < _s._length()) {
            if(_s._char(i) == c) {
                found = true;
            };
            i = i + 1;
        };
        return found;
    };
    method length() {
        return _s._length();
    };
    method char(param i) {
        return _s._char(i);
    };
};