mod approx_search;
mod exact_search;
mod suffix_array_construction;
mod table_gen;
mod types;
mod util;

use approx_search::{approx_search, ApproxSearchParams};
use exact_search::{bwt_search, naive_exact_search};
use suffix_array_construction::construct_suffix_array_naive;
use table_gen::{generate_c_table, generate_o_table};
use types::*;
use util::*;

const ALPHABET: [char; 5] = ['$', 'a', 'c', 'g', 't'];

fn main() {
    let genome = "agatagattcaca$";
    println!("\t=== INPUT IS \"{}\" ===", genome);
    let genome = string_to_ints(genome);

    // Initialize suffix array, O-table, and C-table
    let suffix_array = construct_suffix_array_naive(&genome);
    println!("Suffix array: {:?}", suffix_array);

    let o_table = generate_o_table(&genome, &suffix_array);
    println!("{}", o_table);
    let c_table = generate_c_table(&genome);
    println!("C-table:\n{:?}", c_table);

    // Search with naive binary search
    let search_string_ints = string_to_ints("cgc");
    let search_result = naive_exact_search(&genome, &suffix_array, &search_string_ints);

    //search with bwt exact search
    if search_result == suffix_array.len() {
        println!("No match :(")
    } else {
        println!(
            "Searched for {:?}, with naïve, found at {:?}",
            search_string_ints,
            suffix_array[search_result] + 1
        );
    }

    println!("{:?}", genome);
    let search_string = string_to_ints("att");
    let search_result = bwt_search(&search_string, &o_table, &c_table);
    println!(
        "Searched for {:?}, with bwt-search, found at {:?}",
        search_string, search_result
    );

    //approx search
    let mut reverse_genome = genome.clone();
    reverse_genome.reverse();
    let reverse_suffix_array = construct_suffix_array_naive(&reverse_genome);
    let reverse_o_table = generate_o_table(&reverse_genome, &reverse_suffix_array);
    println!("{}", reverse_o_table);
    println!("c table: {:?}", c_table);

    let params = ApproxSearchParams {
        ref_string: &genome,
        search_string: &string_to_ints("att"),
        o_table: &o_table,
        c_table: &c_table,
        o_table_rev: &reverse_o_table,
        edits_left: 1,
    };
    println!("{:?}", approx_search(params));
}

//TODO: MAKE SA-IS
//TODO: MAKE APPROX SEARCH
//TODO: MAKE SKIPS IN O TABLE
//TODO: MAKE SKIPS IN NUMBERING TABLE
//TODO: WRITE ABOUT IT
