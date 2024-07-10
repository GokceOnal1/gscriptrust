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
        return [1,new Thing(5-2),3];
    };
    
};
assign i = 0;
while (i<10) {
    i = i + 1;
    assign a = new TThing();
    write(a.form()[1].p);
};

