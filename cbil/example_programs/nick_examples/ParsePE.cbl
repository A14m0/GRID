// this file parses a PE file header into a structure

// define a structure for our PE header
struct PEHeader {
    byte[4] signature,
    num machine_id,
    num num_sections,
    byte[4] timedate_stamp,
    byte[4] p_symbol_table,
    num number_of_symtable,
    num opheader_size,
    num characteristics,
    // theres more but im lazy
}


// here we see a very Rust-y way of saying "hey, `PEHeader`s can do this thing"
//
//| here we define this function definition is in relation to a structure
//|
//|    | return type and standard function format follows...
//|    |
//|    |                                     | what structure this function is implemented for
//v    v                                     v
impl PEHeader parse_from_bytes(byte[] data) for PEHeader {
    // bytes 0-3 are for the signature, so we can ignore it
    
    // bytes 4-5 are machine id
    num machine_id = data[4..6].to_num();

    // bytes 6-7 are the number of sections
    num num_sections = data[6..8].to_num();

    // bytes 8-11 are the timedate stamp, so we will deal later
    // bytes 12-15 are the pointer to the symbol table, so also will deal later

    // bytes 16-23 are the number of symbols in the symbol table
    num number_of_symtable = data[16-24];

    // bytes 24-25 are the size of the optional header
    num opheader_size = data[24-26];

    // bytes 26-27 are the characteristics of the file
    num characteristics = data[26-28];

    // theres more but eh. we dont need a full PE parser... yet ;)

    // again, very much a Rust-y way of doing this
    return PEHeader {
        signature: data[0..4],
        machine_id,
        num_sections,
        timedate_stamp: data[8..12],
        p_symbol_table: data[12..16],
        number_of_symtable,
        opheader_size,
        characteristics
    };
}