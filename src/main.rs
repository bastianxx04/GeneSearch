extern crate bincode;

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
use std::fs::{create_dir, File};
use std::io::Write;
use std::path::Path;
use std::time::Instant;
use suffix_array_construction::{construct_suffix_array_naive, suffix_array_induced_sort};
use table_gen::generate_c_table;
use types::*;
use util::*;

const ALPHABET: [char; 5] = ['$', 'A', 'C', 'G', 'T'];
const HG38_1000000: &str = "hg38-1000000";
const HG38_1000: &str = "hg38-1000";

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "sais" => time_sais(),
            "otable" => time_o_table(args),
            "approx" => time_approx(args),
            "exact" => time_exact(args),
            _ => println!("Wut"),
        }
    } else {
        println!("Nothing specified, quitting")
    }
}

pub fn time_sais() {
    let genome = read_and_remap_genome(HG38_1000000);

    let time = Instant::now();
    let sa = suffix_array_induced_sort(&genome);
    let t = time.elapsed();

    println!("SA-IS (length {}) took {} ms", sa.len(), t.as_millis());
}

pub fn time_o_table(args: Vec<String>) {
    let spacing = args[2].parse::<usize>().unwrap();
    let file_name = HG38_1000000;

    let genome = read_and_remap_genome(file_name);
    let suffix_array = get_sa(file_name, &genome, false);

    let time = Instant::now();
    let o_table = OTable::new(&genome, &suffix_array, spacing);
    let t = time.elapsed();
    println!("{}", o_table);
    println!("{}", t.as_millis());
}

pub fn time_approx(args: Vec<String>) {
    let spacing = args[2].parse::<usize>().unwrap();
    let genome_file_name = HG38_1000000;

    let genome = read_genome(genome_file_name);
    let remapped_genome = remap_reference(&genome);

    let suffix_array = get_sa(genome_file_name, &remapped_genome, false);
    let o_table = get_o_table(
        genome_file_name,
        &remapped_genome,
        &suffix_array,
        spacing,
        false,
    );
    let c_table = generate_c_table(&remapped_genome);

    let reverse_genome: String = genome.chars().rev().collect();
    let reverse_remapped = remap_reference(&reverse_genome);

    // TODO: Gem også reverse suffix array til disk
    let reverse_suffix_array = get_sa(genome_file_name, &reverse_remapped, true);
    let reverse_o_table = get_o_table(
        &genome_file_name,
        &reverse_remapped,
        &reverse_suffix_array,
        spacing,
        true,
    );

    // TODO: Tag et faktisk read i stedet for denne string
    let query = remap_query("AATAAACCTTACCTAGCA");

    let params = ApproxSearchParams {
        reference: &remapped_genome,
        query: &query,
        o_table: &o_table,
        c_table: &c_table,
        rev_o_table: &reverse_o_table,
        edits: 1,
    };

    let time = Instant::now();
    let results = approx_search(params);
    let t = time.elapsed();

    println!("{:?}", results);
    println!("{}", t.as_nanos());
}

pub fn time_exact(args: Vec<String>) {
    let spacing = args[2].parse::<usize>().unwrap();
    let file_name = HG38_1000000;

    // TODO: Tag et faktisk read i stedet for denne string
    let query = remap_query("AATAAACCTTACCTAGCACTCCATCATGTCTTATGGCGCGTGATTTGCCCCGGACTCAGGCAAAACCC");

    let genome = read_and_remap_genome(file_name);
    let suffix_array = get_sa(file_name, &genome, false);
    let o_table = get_o_table(file_name, &genome, &suffix_array, spacing, false);
    let c_table = generate_c_table(&genome);

    let time = Instant::now();
    let result = bwt_search(&query, &o_table, &c_table);
    let t = time.elapsed();

    println!("{:?}", result);
    println!("{}", t.as_nanos());
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
    let genome_string = try_read_genome(HG38_1000)?;

    // Run the algs
    let genome = remap_reference(&genome_string);

    // Initialize suffix array, O-table, and C-table
    let suff_and_table_start = Instant::now();

    let suffix_array = suffix_array_induced_sort(&genome);

    let o_table = OTable::new(&genome, &suffix_array, 10);
    let c_table = generate_c_table(&genome);

    let suff_and_table_time = suff_and_table_start.elapsed().as_nanos();
    println!("Finished generating tables...");

    let query = remap_query("AATAAACCTTACCTAGCACTCCATCATGTCTTATGGCGCGTGATTTGCCCCGGACTCAGGCAAAACCC");

    //search with bwt exact search
    let exact_time_start = Instant::now();

    let (exact_start, exact_end) = bwt_search(&query, &o_table, &c_table);

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
        query: &query,
        o_table: &o_table,
        c_table: &c_table,
        rev_o_table: &reverse_o_table,
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
        query.len(),
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
