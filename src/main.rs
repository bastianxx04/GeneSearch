mod suffix_array_construction;
mod exact_search;
//mod approx_search;
mod suffix_array;

use suffix_array_construction::construct_suffix_array_naive;
use exact_search::{naive_exact_search, exact_search};
use suffix_array::SuffixArray;

const ALPHABET: [char; 5] = ['$', 'a', 'c', 'g', 't'];

fn main() {
    //OUR CURRENT ALPHABET IS A, C, G, T
    let input_string = "cagagagagagagat";
    println!("  === INPUT IS \"{}\" ===", input_string);

    let mut input_array: Vec<_> = input_string.chars().collect();
    input_array.push('$');

    //Make the Data Array   [Suffix], num, [Rank Table]
    let suffix_array = construct_suffix_array_naive(&input_array);

    // Search with naive binary search
    let search_string = vec!['a', 'g', 'a'];
    let search_result = naive_exact_search(&suffix_array, &search_string);
    
    if search_result == suffix_array.array.len() {
        println!("No match :(")
    } else {   
        println!("Searched for {:?}, with naïve, found at {:?}", search_string, suffix_array.array[search_result]+1);
    }

    let o_table = generate_o_table(&suffix_array);
    println!("O-table:\n{:?}", o_table);
    let c_table = generate_c_table(&suffix_array);
    println!("C-table:\n{:?}", c_table);
    
    let search_string = vec!['a', 'g', 'a'];
    let search_result = exact_search( &search_string, &o_table, &c_table);
    println!("Searched for {:?}, with bwt-search, found at {:?}", search_string , search_result)
}

fn generate_o_table(suffix_array: &SuffixArray) -> Vec<Vec<usize>> {
    //ORDER: A, C, G, T
    let mut old_rank_table = vec![0, 0, 0, 0, 0];
    let mut new_bwt_output = Vec::new();
    for i in 0..suffix_array.array.len() {
        let mut new_rank_table = old_rank_table.clone();
        match prev_elem(suffix_array.string, suffix_array.array[i]) {
            '$' => new_rank_table[0] += 1,
            'a' => new_rank_table[1] += 1,
            'c' => new_rank_table[2] += 1,
            'g' => new_rank_table[3] += 1,
            't' => new_rank_table[4] += 1,
            _ => {}
        }

        old_rank_table = new_rank_table.clone();
        new_bwt_output.push(new_rank_table.clone())
    }
    new_bwt_output
}

fn generate_c_table(sa: &SuffixArray) -> Vec<usize> {
    /*
    let mut acc = 0;
    ALPHABET.iter()
    .map(|&c| sa.array.iter().position(|&r| sa.string[r] == c).unwrap_or(0))
    .map(|x| {
        acc = acc + x;
        acc
    })
    .collect()
    
    let mut c_table = vec![0];
    for c in ALPHABET[1..].iter() {
        let i = sa.array.iter()
        .position(|&r| sa.string[r] == *c)
        .unwrap_or(0);
        
        let v = c_table.last().unwrap() + i;
        c_table.push(v);
    }
    c_table
    */
    
    let mut counts = vec![0;5];
    for &c in sa.string {
        let i = ALPHABET.iter().position(|&x| x == c).expect("Char was not in alphabet");
        counts[i] += 1;
    }

    let mut c_table = vec![0];
    for i in 1..ALPHABET.len() {
        let v = c_table.last().unwrap() + counts[i - 1];
        c_table.push(v);
    }
    c_table
}

fn next_elem<T>(array: &[T], i: usize) -> &T {
    &array[i + 1 % array.len()]
}

fn prev_elem<T>(array: &[T], i: usize) -> &T {
    &array[(array.len() + i - 1) % array.len()]
}

//TODO: MAKE SA-IS
//TODO: MAKE APPROX SEARCH
//TODO: MAKE SKIPS IN O TABLE
//TODO: MAKE SKIPS IN NUMBERING TABLE
//TODO: WRITE ABOUT IT
