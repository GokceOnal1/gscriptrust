funct waicy_thing(param protein_sequence, param ranges, param amino_acids) {
    assign i = 0;
    assign modified_sequence = protein_sequence;
    while( i < length(ranges) ) {
        assign r = random_int(0, length(amino_acids)-1);
        modified_sequence = replace(modified_sequence, ranges[i], amino_acids[r]);
        i = i + 1;
    };
    return modified_sequence;
};

write(waicy_thing("MELGFSWVFLVTLLNGIQCEVKLVESGGGLVQPGGSLRLSCATSGFTFTDYYMSWVRQPPGKALEWLGFIRNKANGYTTEYSASVKGRFTISRDNSQSILYLQMNTLRAEDSATYYCARDNGDYDYERFAYWGQGTLVTV", [49, 50, 51, 52, 53, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130], "ACDEFGHIKLMNPQRSTVWY"));