funct waicy_thing(param protein_sequence, param ranges, param amino_acids) {
    assign i = 0;
    while( i < ranges.length ) {
        assign r = random_int(0, amino_acids.length-1);
        protein_sequence.replace(ranges[i], amino_acids.get(r) );
        i = i + 1;
    };
    return protein_sequence;
};

write(waicy_thing("AOIWWDJEGOIN", [1,2,3,4], "OAIWJDOIJAWD"));