\ ============================================================ \
\ basic rock paper scissors \
write("welcome to rock paper scissors");

assign input = "none";
while(input != "e") {
    write("enter choice: r/p/s or write e to quit");
    input = read();
    if(input == "r") {
        write("you picked rock");
    } else if(input == "p") {
        write("you picked paper");
    } else if(input == "s") {
        write("you picked scissors");
    } else if(input == "e") {
        write("quitting now");
    } else {
        write("invalid input");
    };
};
\ end rock paper scissors example \
\ ============================================================ \
\ lists example \
assign list = [1, ["chode", "gyatt"], 3];
write(list);
list[1][0] = 5;
write(list);
\ end lists example \
\ ============================================================ \
\ blueprints example \
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

assign myFirstCar = new Car("Toyota", "Prius", "grey", 2010);
assign myCar = new Car(myFirstCar, "Forester", "green", 2016);
write(myCar);
write(myFirstCar);
\ end blueprints example \
\ ============================================================ \
\ blueprints example 2 \
blueprint Thing {
    prop p;
    method create(param pp) {
        p = pp;
    };
    method speak(param input) {
        write("aris ", p, " ", input);
    };
};
funct a(param th) {
    th.p = 5;
    write(th.p);
};

assign thing = new Thing(1);
a(thing);
write(thing.p);
\ end blueprints example 2 \
\ ============================================================ \