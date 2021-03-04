mod suffix_array_construction;
mod exact_search;
mod approx_search;
mod types;
mod table_gen;
mod util;

use suffix_array_construction::construct_suffix_array_naive;
use exact_search::{naive_exact_search, exact_search};
use approx_search::approx_search;
use types::*;
use table_gen::{generate_c_table, generate_o_table};
use util::*;

const ALPHABET: [char; 5] = ['$', 'a', 'c', 'g', 't'];

fn main() {
    let genome = "aggagc$";
    println!("\t=== INPUT IS \"{}\" ===", genome);
    let mut genome_vec = string_to_ints(genome);

    // Initialize suffix array, O-table, and C-table
    let suffix_array = construct_suffix_array_naive(&genome_vec);

    let o_table = generate_o_table(&suffix_array);
    print_o_table(&o_table, &genome_vec, &suffix_array.array);
    let c_table = generate_c_table(&suffix_array);
    println!("C-table:\n{:?}", c_table);

    // Search with naive binary search
    let search_string_ints = string_to_ints("cgc");
    let search_result = naive_exact_search(&suffix_array, &search_string_ints);
    
    //search with bwt exact search
    if search_result == suffix_array.array.len() {
        println!("No match :(")
    } else {   
        println!("Searched for {:?}, with naïve, found at {:?}", search_string_ints, suffix_array.array[search_result]+1);
    }

    let search_string = vec!['a', 'g', 'a'];
    let search_result = exact_search( &search_string, &o_table, &c_table);
    println!("Searched for {:?}, with bwt-search, found at {:?}", search_string , search_result);

    //approx search
    genome_vec.reverse();
    let reverse_suffix_array = construct_suffix_array_naive(&genome_vec);
    let reverse_o_table = generate_o_table(&reverse_suffix_array);
    print_o_table(&reverse_o_table, &genome_vec, &reverse_suffix_array.array);
    println!("c table: {:?}", c_table);
    approx_search(&genome_vec, &search_string_ints, &reverse_o_table, &c_table, 1);
    genome_vec.reverse();

}

fn string_to_ints(s: &str) -> Vec<u8> {
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

//TODO: MAKE SA-IS
//TODO: MAKE APPROX SEARCH
//TODO: MAKE SKIPS IN O TABLE
//TODO: MAKE SKIPS IN NUMBERING TABLE
//TODO: WRITE ABOUT IT
