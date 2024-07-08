blueprint Thing {
    prop p;
    method create(param pp) {
        p = pp;
    };
};
funct test() {
    return [1,new Thing(1),3]
};
write(test()[1].p);