blueprint _list {
    prop _l;

    \ create() is included by convention \
    method create() {};

    method length() {
        return _l._length();
    };
};