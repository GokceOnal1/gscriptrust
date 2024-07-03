blueprint Car {
    prop brand;
    prop model;
    prop color;
    prop year;
    method create(param b, param m, param c, param y) {
        brand = b;
        model = m;
        color = c;
        year = y;
    };
};

assign myCar = new Car("Subaru", "Forester", "green", 2016);
write(myCar);