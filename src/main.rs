#![feature(test)]

extern crate test;

mod approx_search;
mod exact_search;
mod suffix_array_construction;
mod table_gen;
mod types;
mod util;

use chrono::Local;
use std::{
    fs::{File, create_dir},
    io::{BufReader, Write, Read},
    time::Instant,
    path::Path,
};
use approx_search::{approx_search, ApproxSearchParams};
use exact_search::{bwt_search};
use suffix_array_construction::construct_suffix_array_naive;
use table_gen::{generate_c_table, generate_o_table};
use types::*;
use util::*;

const ALPHABET: [char; 5] = ['$', 'A', 'C', 'G', 'T'];

fn main() {
    match log_performance() {
        Ok(_) => println!("Finished cleanly."),
        Err(error) => println!("Error: {}", error),
    }
}

pub fn log_performance() -> std::io::Result<()> {
    // Ensure that the directory exists
    let results_path = Path::new("./results/");
    if !results_path.exists() {
        match create_dir(results_path) {
            Ok(_) => println!("Created \"results\" directory"),
            Err(error) => panic!("Could not create directory: {}", error),
        }
    }
    
    // Create the file name
    let local = Local::now();
    let filename = results_path.join(local.format("Result_%Y-%m-%dT%H.%M.txt").to_string());
    println!("{:?}", filename);
    let mut file = File::create(filename)?;

    // Read the genome file
    let genome_file = File::open("resources/hg38-1000.fa")?;
    let mut buf_reader = BufReader::new(genome_file);
    let mut genome_string_raw = String::new();
    buf_reader.read_to_string(&mut genome_string_raw)?;
    let mut genome_string = genome_string_raw.replace('\n', "");
    genome_string = genome_string.replace("> chr1", "");
    genome_string.push('$');

    // Run the algs
    let genome = string_to_ints(&genome_string);

    // Initialize suffix array, O-table, and C-table
    let suff_and_table_start = Instant::now();
    
    let suffix_array = construct_suffix_array_naive(&genome);
    
    let o_table = generate_o_table(&genome, &suffix_array);
    let c_table = generate_c_table(&genome);
    
    let suff_and_table_time = suff_and_table_start.elapsed().as_nanos();
    println!("Finished generating tables...");
    
    
    let search_string_ints = string_to_ints("AATAAACCTTACCTAGCACTCCATCATGTCTTATGGCGCGTGATTTGCCCCGGACTCAGGCAAAACCC");
    
    //search with bwt exact search
    let exact_time_start = Instant::now();
    
    let (exact_start, exact_end) = bwt_search(&search_string_ints, &o_table, &c_table);
    
    let exact_time = exact_time_start.elapsed().as_nanos();
    println!("Finished exact search...");
    
    let mut exact_matches = vec![];
    for i in exact_start..(exact_end + 1) {
        exact_matches.push(suffix_array[i]);
    }

    //approx search
    let approx_start = Instant::now();
    
    let mut reverse_genome = genome.clone();
    reverse_genome.reverse();
    let reverse_suffix_array = construct_suffix_array_naive(&reverse_genome);
    let reverse_o_table = generate_o_table(&reverse_genome, &reverse_suffix_array);
    
    let params = ApproxSearchParams {
        reference: &genome,
        query: &search_string_ints,
        o_table: &o_table,
        c_table: &c_table,
        o_rev_table: &reverse_o_table,
        edits: 1,
    };
    
    let approx_search_result = approx_search(params);
    
    let approx_time = approx_start.elapsed().as_nanos();
    println!("Finished approx search...");

    let mut approx_matches = vec![];
    for (start, end, cigar, edits) in approx_search_result.iter() {
        let mut indices = vec![];
        for i in (*start)..(*end) {
            indices.push(suffix_array[i]);
        }
        
        let mut formatted_cigar = String::new();
        let mut cigar_format_buffer: (usize, char) = (1, cigar.chars().next().unwrap());
        for i in 1..cigar.len() {
            let (num, curr_char) = cigar_format_buffer;
            let c = cigar.chars().nth(i).unwrap();
            if c == curr_char {
                cigar_format_buffer = (num + 1, curr_char);
            } else {
                formatted_cigar.push_str(&format!("{}{}", num, curr_char));
                cigar_format_buffer = (1, c)
            }
        }
        formatted_cigar.push_str(&format!("{}{}", cigar_format_buffer.0, cigar_format_buffer.1));
        
        approx_matches.push(format!("
    Matches on indices: {:?}
    Cigar for matches: {:?}
    Edits: {}", indices, formatted_cigar, edits));
    }

    let file_string = format!("     === TESTS ===
Searching in a genome of length: {}
Searching for a string with length: {}\n
Table generation took {} ns ({} s) 
Exact Search took {} ns ({} s) and yielded {:?} 
Approx search took {} ns ({} s) and yielded {} \n
    Total execution time: {} s
    ", 
    genome.len(), search_string_ints.len(),
    suff_and_table_time, suff_and_table_time/1000000000, 
    exact_time, exact_time/1000000000, exact_matches, 
    approx_time, approx_time/1000000000, approx_matches.join("\n"), 
    (suff_and_table_time+exact_time+approx_time)/1000000000);

    file.write_all(file_string.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_entire_system(b: &mut Bencher) {
        b.iter(|| {
            (0..1000).fold(0, |old, new| old ^ new)
        })
    }
}

//TODO: MAKE SA-IS
//TODO: MAKE APPROX SEARCH
//TODO: MAKE SKIPS IN O TABLE
//TODO: MAKE SKIPS IN NUMBERING TABLE
//TODO: WRITE ABOUT IT
