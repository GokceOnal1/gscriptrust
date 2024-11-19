funct waicy_thing(param protein_sequence, param ranges, param amino_acids) {
    assign i = 0;
    assign modified_sequence = "";
    while( i < length(ranges) ) {
        assign r = random_int(0, length(amino_acids)-1);
        modified_sequence = replace(protein_sequence, ranges[i], amino_acids[r]);
        i = i + 1;
    };
    return modified_sequence;
};

write(waicy_thing("AOIWWDJEGOIN", [1,2,3,4], "XPLQRSTUV"));