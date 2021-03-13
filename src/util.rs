use crate::ALPHABET;

pub fn print_o_table(o: &Vec<Vec<usize>>, x: &Vec<u8>, sa: &Vec<usize>) {
    println!("O-table:");

    // Header row
    print!("{:>3}", ' ');

    for i in 0..sa.len() {
        let j = bwt(x, sa, i);
        let c = ALPHABET[j as usize];
        print!("{:>3}", c);
    }
    println!();

    // Data rows
    for (i, c) in ALPHABET.iter().enumerate() {
        print!("{:>3}", c);
        for j in 0..x.len() {
            let v = o[j][i];
            print!("{:>3}", v);
        }
        println!();
    }
}

pub fn string_to_ints(s: &str) -> Vec<u8> {
    s.chars()
        .map(|c| match c {
            '$' => 0,
            'a' => 1,
            'c' => 2,
            'g' => 3,
            't' => 4,
            _ => panic!("Bad string"),
        })
        .collect()
}

pub fn bwt(x: &[u8], sa: &[usize], i: usize) -> u8 {
    let x_index = sa[i];
    if x_index == 0 {
        x[x.len() - 1]
    } else {
        x[x_index - 1]
    }
}