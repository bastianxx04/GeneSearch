use std::fs::File;
use std::io::prelude::*;
use chrono::prelude::*;
use std::io::BufReader;
use std::time::Instant;
use crate::{
    approx_search::{approx_search, ApproxSearchParams},
    exact_search::{bwt_search, naive_exact_search},
    suffix_array_construction::construct_suffix_array_naive,
    table_gen::{generate_c_table, generate_o_table},
    types::*,
    util::*,
};

pub fn runtests() -> std::io::Result<()> {
    let local = Local::now();
    let filename = local.format("results/Result_%Y-%m-%dT%H.%M.txt").to_string();
    println!("{}", filename);
    let mut file = File::create(filename)?;

    let genome_file = File::open("resources/hg38-1000.fa")?;
    let mut buf_reader = BufReader::new(genome_file);
    let mut genome_string_raw = String::new();
    buf_reader.read_to_string(&mut genome_string_raw)?;
    let mut genome_string = genome_string_raw.replace('\n', "");
    genome_string = genome_string.replace("> chr1", "");

    let genome = string_to_ints(&genome_string);

    // Initialize suffix array, O-table, and C-table
    let suff_and_table_start = Instant::now();
    
    let suffix_array = construct_suffix_array_naive(&genome);
    
    let o_table = generate_o_table(&genome, &suffix_array);
    let c_table = generate_c_table(&genome);
    
    let suff_and_table_time = suff_and_table_start.elapsed().as_nanos();
    
    
    let search_string_ints = string_to_ints("ATCTAGAGAAACAGTTTTGAGCCCTTTTATTTTGCTCAACAGT");
    
    //search with bwt exact search
    let exact_start = Instant::now();
    
    let exact_search_result = bwt_search(&search_string_ints, &o_table, &c_table);
    
    let exact_time = exact_start.elapsed().as_nanos();
    
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
    
    let mut file_string = format!("=== TESTS ===\n
    Table generation took {:?} ns \n
    Exact Search took {:?} ns and yielded {:?} \n
    Approx search took {:?} ns and yielded {:?}", suff_and_table_time, exact_time, exact_search_result, approx_time, approx_search_result);

    file.write_all(file_string.as_bytes())?;

    Ok(())
}
