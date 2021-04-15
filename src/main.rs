#![feature(test)]
extern crate test;

mod approx_search;
mod exact_search;
mod o_table;
mod suffix_array_construction;
mod table_gen;
mod types;
mod util;

use approx_search::{approx_search, ApproxSearchParams};
use chrono::Local;
use exact_search::bwt_search;
use o_table::OTable;
use std::{
    fs::{create_dir, File},
    io::{BufReader, Read, Write},
    path::Path,
    time::{Duration, Instant},
};
use suffix_array_construction::{construct_suffix_array_naive, suffix_array_induced_sort};
use table_gen::generate_c_table;
use types::*;
use util::*;
// use serde_json;
// use chrono::serde::ts_seconds::serialize;

const ALPHABET: [char; 5] = ['$', 'A', 'C', 'G', 'T'];
const HG38_1000_PATH: &str = "resources/genomes/hg38-1000.fa";
const HG38_1000000_PATH: &str = "resources/genomes/hg38-1000000.fa";

const HG38_1000000_SA: &str = "resources/sa/hg38-1000000.txt";

fn main() {
    let a = read_sa(HG38_1000000_SA);
    let cmd_line: Vec<String> = std::env::args().collect();

    if cmd_line.len() > 1 {
        match cmd_line[1].as_str() {
            "sais" => {
                let (t, len) = time_sais(HG38_1000000_PATH);
                println!("SA-IS (length {}) took {} ms", len, t.as_millis());
            },
            "otable" => {
                let skips = &cmd_line[2].parse::<usize>().unwrap();
                let length = &cmd_line[3].parse::<usize>().unwrap();
                let genome_string = read_genome(HG38_1000000_PATH).slice(0..length);

                let (t, o) = time_otable(&remap_string(&genome_string), &read_sa(HG38_1000000_SA), *skips);
                println!("{}", t.as_millis());
            }
            "approx" => {
                let skips = &cmd_line[2].parse::<usize>().unwrap();
            },
            "exact" => {
                let skips = &cmd_line[2].parse::<usize>().unwrap();
            },
            _ => println!("Wut")
        }
    } else {
        println!("Nothing specified, quitting")
    }
}

fn read_sa(path: &str) -> Vec<usize> {
    // match File::open(path) {
    //     Ok(file) => {
    //         println!("Here");
    //         //let mut buf_reader = BufReader::new(file);
    //         //let mut suffix_array = String::new();
    //         //buf_reader.read_to_string(&mut suffix_array);
    //         let return_vec: Vec<usize> = serde_json::from_reader(file).expect("error reading json");
    //         //return suffix_array;
    //         return return_vec;
    //     }
    //     Err(_) => {
    //         let genome = match read_genome(HG38_1000000_PATH) {
    //             Ok(genome) => genome,
    //             Err(_) => panic!("could not read genome"),
    //         };

    //         let genome = remap_string(&genome);
    //         let sa = suffix_array_induced_sort(&genome);
    //         println!("har regnet");
    //         let mut f = File::create(path).unwrap();
    //         let a = serialize(sa, serde::Serialize);
    //         let s = serde_json::to_writer(&f, &a).unwrap();
    //         return sa
    //     }
    // }
    return vec![0];
}

pub fn time_otable<'a>(reference: &'a[u8], sa: &'a Vec<usize>, skips: usize) -> (Duration, OTable<'a>) {
    let time = Instant::now();
    let o_table = OTable::new(&reference, &sa, skips);
    (time.elapsed(), o_table)
}

pub fn time_approx(skips: usize) {
    let time = Instant::now();
    todo!()
}

pub fn time_exact(skips: usize) {
    let time = Instant::now();
    todo!()
}

pub fn time_sais(path: &str) -> (Duration, usize) {
    let genome = match read_genome(path) {
        Ok(genome) => genome,
        Err(_) => panic!("could not read genome"),
    };

    let genome = remap_string(&genome);

    let time = Instant::now();
    let sa = suffix_array_induced_sort(&genome);
    (time.elapsed(), sa.len())
}

pub fn read_genome(path: &str) -> std::io::Result<String> {
    let genome_file = File::open(path)?;
    let mut buf_reader = BufReader::new(genome_file);
    let mut genome_string_raw = String::new();
    buf_reader.read_to_string(&mut genome_string_raw)?;
    let mut genome_string = genome_string_raw.replace('\n', "");
    genome_string = genome_string.replace("> chr1", "");
    genome_string.push('$');

    Ok(genome_string)
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
    let genome_string = read_genome(HG38_1000_PATH)?;

    // Run the algs
    let genome = remap_string(&genome_string);

    // Initialize suffix array, O-table, and C-table
    let suff_and_table_start = Instant::now();

    let suffix_array = suffix_array_induced_sort(&genome);

    let o_table = OTable::new(&genome, &suffix_array, 10);
    let c_table = generate_c_table(&genome);

    let suff_and_table_time = suff_and_table_start.elapsed().as_nanos();
    println!("Finished generating tables...");

    let search_string_ints =
        remap_string("AATAAACCTTACCTAGCACTCCATCATGTCTTATGGCGCGTGATTTGCCCCGGACTCAGGCAAAACCC");

    //search with bwt exact search
    let exact_time_start = Instant::now();

    let (exact_start, exact_end) = bwt_search(&search_string_ints, &o_table, &c_table);

    let exact_time = exact_time_start.elapsed().as_nanos();
    println!("Finished exact search...");

    let mut exact_matches = vec![];
    for item in suffix_array.iter().take((exact_end) + 1).skip(exact_start) {
        exact_matches.push(item);
    }

    //approx search
    let approx_start = Instant::now();

    let mut reverse_genome = genome.clone();
    reverse_genome.reverse();
    let reverse_suffix_array = construct_suffix_array_naive(&reverse_genome);
    let reverse_o_table = OTable::new(&reverse_genome, &reverse_suffix_array, 10);

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
        for item in suffix_array.iter().take(*end).skip(*start) {
            indices.push(item);
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
        formatted_cigar.push_str(&format!(
            "{}{}",
            cigar_format_buffer.0, cigar_format_buffer.1
        ));

        approx_matches.push(format!(
            "
    Matches on indices: {:?}
    Cigar for matches: {:?}
    Edits: {}",
            indices, formatted_cigar, edits
        ));
    }

    let file_string = format!(
        "     === TESTS ===
Searching in a genome of length: {}
Searching for a string with length: {}\n
Table generation took {} ns ({} s) 
Exact Search took {} ns ({} s) and yielded {:?} 
Approx search took {} ns ({} s) and yielded {} \n
    Total execution time: {} s
    ",
        genome.len(),
        search_string_ints.len(),
        suff_and_table_time,
        suff_and_table_time / 1000000000,
        exact_time,
        exact_time / 1000000000,
        exact_matches,
        approx_time,
        approx_time / 1000000000,
        approx_matches.join("\n"),
        (suff_and_table_time + exact_time + approx_time) / 1000000000
    );

    file.write_all(file_string.as_bytes())?;

    Ok(())
}

//TODO: MAKE SA-IS
//TODO: MAKE SKIPS IN O TABLE
//TODO: MAKE SKIPS IN NUMBERING TABLE
//TODO: WRITE ABOUT IT
