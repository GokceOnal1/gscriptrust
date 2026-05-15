\ DOES NOT WORK AS INTENDED \

blueprint test {
    prop b;
    method create(param bb) {b=bb;};
    method add_one() {
        b = b + 1;
        write(b);
    };
};

assign a = new test(5);
a.add_one();
write(a.b);