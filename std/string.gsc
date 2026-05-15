blueprint _string {
    prop _s;

    \ create() is included by convention \
    method create() {};

    method length() {
        return _s._length();
    };
    
    method char_at(param i) {
        return _s._char(i);
    };

    method contains(param c) {
        assign i = 0;
        while(i < _s._length()) {
            if(_s._char(i) == c) {
                return true;
            };
            i = i + 1;
        };
        return false;
    };

    method index_of(param c) {
        assign i = 0;
        while (i < _s._length()) {
            if(_s._char(i) == c) {
                return i;
            };
            i = i + 1;
        };
        return -1;
    };
};