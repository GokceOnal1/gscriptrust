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