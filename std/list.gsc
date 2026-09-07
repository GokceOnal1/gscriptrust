blueprint _list {
    prop _l;

    method create() {};

    method length() {
        return _l._length();
    };
};