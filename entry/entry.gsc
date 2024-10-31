assign num = random_int(1, 100);
assign guess = -1;

write("Welcome to the Guessing Game!");

while(guess != num) {
    write("Enter guess: ");

    guess = to_int(read());

    if (guess > num) {
        write("Too high...");
    } else if (guess < num) {
        write("Too low...");
    } else {
        write("You got it! The number was ", num, "!");
    };
};
