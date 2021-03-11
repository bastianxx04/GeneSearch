use crate::{ALPHABET, table_gen::prev_elem};

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
