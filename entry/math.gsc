assign pi = 3.14;

funct exp(param base, param pow) {
    if(pow <= 0) {
        return 1;
    } else {
        return base * exp(base, pow-1);
    };
};

blueprint AdvancedNum {
    prop base;
    method create(param b) {
        base = b;
    };
};

funct thing() {
    return new AdvancedNum(5);
};