blueprint Thing {
    prop p;
    method create(param pp) {
        p = pp;
    };
    method w() {
        write("hello");
    };
};
blueprint TThing {
    method create() {

    };
    method form() {
        return [0, new Thing(5)];
    };
    
};
assign t = new Thing(5);
t.p.z.o = 3;
