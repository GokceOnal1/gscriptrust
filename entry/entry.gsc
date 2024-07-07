blueprint Thing {
    prop p;
    method create(param pp) {
        p = pp;
    };
    method speak(param input) {
        write("aris ", p, " ", input);
    };
};
assign t = new Thing(1);
assign a = 10;
write(t.b);