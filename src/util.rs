use crate::types::OTable;
use crate::{ALPHABET};
use crate::table_gen::prev_elem;

pub fn print_o_table(o: &Vec<Vec<usize>>, s: &Vec<u8>, sa: &Vec<usize>) {
    println!("O-table:");

    // Header row
    print!("{:>3}", ' ');

    for i in sa {
        let j = prev_elem(s, *i);
        let c = ALPHABET[usize::from(*j)];
        print!("{:>3}", c);
    }
    println!();

    // Data rows
    for (i, c) in ALPHABET.iter().enumerate() {
        print!("{:>3}", c);
        for j in 0..s.len() {
            let v = o[j][i];
            print!("{:>3}", v);
        }
        println!();
    }
}

/*
for hvert bogstav, index i alfabet {
    for hver liste i table,
        print liste[index]
}
*/


/*
O-table for string [0, 4, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 2]:

[
    [0, 0, 1, 0, 0],
    [0, 0, 1, 1, 0],
    [0, 0, 1, 2, 0],
    [0, 0, 1, 3, 0],
    [0, 0, 1, 4, 0],
    [0, 0, 1, 5, 0],
    [0, 0, 1, 6, 0],
    [0, 0, 1, 6, 1],
    [0, 1, 1, 6, 1],
    [0, 2, 1, 6, 1],
    [0, 3, 1, 6, 1],
    [0, 4, 1, 6, 1],
    [0, 5, 1, 6, 1],
    [0, 6, 1, 6, 1],
    [0, 7, 1, 6, 1],
    [1, 7, 1, 6, 1]
]
*/