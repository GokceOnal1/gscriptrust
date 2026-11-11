funct fib(param n) {
    if (n < 2) {
        return n;
    };
    return fib(n-1)+fib(n-2);
};

write(fib(40))