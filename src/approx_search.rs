use crate::DTable;
use crate::ALPHABET;
use crate::{types::OTable, CTable};
use std::collections::HashSet;
use std::convert::TryFrom;

#[derive(Copy, Clone)]
pub struct ApproxSearchParams<'a> {
    pub reference: &'a [u8],
    pub query: &'a [u8],
    pub o_table: &'a OTable<'a>,
    pub c_table: &'a CTable,
    pub o_rev_table: &'a OTable<'a>,
    pub edits: usize,
}

/// Approximative search
pub fn approx_search(params: ApproxSearchParams) -> HashSet<(usize, usize, String, usize)> {
    let d_table = calculate_d_table(
        params.reference,
        params.query,
        params.c_table,
        params.o_rev_table,
    );
    let o_table = params.o_table;
    let c_table = params.c_table;

    let left = 0;
    let right = params.reference.len();
    let i = (params.query.len() - 1) as i32;

    let mut result = HashSet::new();

    // M-operations
    for a in 1..ALPHABET.len() {
        let new_left = c_table[a] + o_table.get(a as u8, left);
        let new_right = c_table[a] + o_table.get(a as u8, right);

        let edit_cost: i32 = if a == params.query[i as usize].into() {
            0
        } else {
            1
        };
        let edits_left = (params.edits as i32) - edit_cost;
        if edits_left < 0 {
            continue;
        }
        if new_left >= new_right {
            continue;
        }

        result = result
            .union(&inexact_recursion(
                params.query,
                i - 1,
                edits_left,
                new_left,
                new_right,
                &d_table,
                o_table,
                c_table,
                String::from(if edit_cost == 0 { "M" } else { "S" }),
                edit_cost as usize,
            ))
            .cloned()
            .collect();
    }

    // I-operation
    result = result
        .union(&inexact_recursion(
            params.query,
            i - 1,
            (params.edits as i32) - 1,
            left,
            right,
            &d_table,
            o_table,
            c_table,
            String::from("I"),
            1,
        ))
        .cloned()
        .collect();

    result
}

fn calculate_d_table(
    reference: &[u8],
    query: &[u8],
    c_table: &[usize],
    o_rev_table: &OTable,
) -> DTable {
    let mut start = 1;
    let mut end = reference.len() - 1;
    let mut edits_left = 0;
    let mut d_table: DTable = Vec::new();

    for c in query {
        let current_symbol = usize::from(*c);
        start = c_table[current_symbol] + o_rev_table.get(current_symbol as u8, start - 1) + 1;
        end = c_table[current_symbol] + o_rev_table.get(current_symbol as u8, end);
        if start > end {
            start = 1;
            end = reference.len() - 1;
            edits_left += 1;
        }
        d_table.push(edits_left);
    }

    d_table
}

fn inexact_recursion(
    query: &[u8],
    i: i32,
    edits_left: i32,
    left: usize,
    right: usize,
    d_table: &[usize],
    o_table: &OTable,
    c_table: &[usize],
    cigar: String,
    edits_total: usize,
) -> HashSet<(usize, usize, String, usize)> {
    // println!("entered recursive loop at level: {}", i);
    let lower_limit = match usize::try_from(i) {
        Ok(value) => d_table[value],
        Err(_) => 0,
    };

    if edits_left < lower_limit as i32 {
        return HashSet::new();
    }

    let mut result_set = HashSet::new();

    if i < 0 {
        result_set.insert((left, right, cigar, edits_total));
        return result_set;
    }

    // Match/substitute operation
    let current_char = query[i as usize];
    let mut new_left;
    let mut new_right;

    for c in 1..ALPHABET.len() {
        let c = c as u8;
        new_left = c_table[c as usize] + o_table.get(c, left);
        new_right = c_table[c as usize] + o_table.get(c, right);
        let edit_cost = if c == current_char { 0 } else { 1 };

        if (edits_left - edit_cost) < 0 {
            continue;
        };
        if new_left >= new_right {
            continue;
        };

        result_set = result_set
            .union(&inexact_recursion(
                query,
                i - 1,
                edits_left - edit_cost,
                new_left,
                new_right,
                d_table,
                o_table,
                c_table,
                format!("{}{}", if edit_cost == 0 { "M" } else { "S" }, cigar),
                edits_total + edit_cost as usize,
            ))
            .cloned()
            .collect()
    }

    // Insertion operation
    result_set = result_set
        .union(&inexact_recursion(
            query,
            i - 1,
            edits_left - 1,
            left,
            right,
            d_table,
            o_table,
            c_table,
            format!("{}{}", "I", cigar),
            edits_total + 1,
        ))
        .cloned()
        .collect();

    // Deletion operation
    for c in 1..ALPHABET.len() {
        let c = c as u8;

        new_left = c_table[c as usize] + o_table.get(c, left);
        new_right = c_table[c as usize] + o_table.get(c, right);

        if left >= right {
            continue;
        };

        result_set = result_set
            .union(&inexact_recursion(
                query,
                i,
                edits_left - 1,
                new_left,
                new_right,
                d_table,
                o_table,
                c_table,
                format!("{}{}", "D", cigar),
                edits_total + 1,
            ))
            .cloned()
            .collect();
    }
    result_set
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        bwm, bwt, construct_suffix_array_naive, generate_c_table, generate_o_table, read_genome,
        string_to_ints, HG38_1000_PATH,
    };
    use test::Bencher;

    #[test]
    fn test_att_with_1_edit() {
        let reference = string_to_ints("AGATAGATTCACA$");
        let suffix_array = construct_suffix_array_naive(&reference);

        let reverse_reference: Vec<u8> = reference.iter().rev().map(|&x| x).collect();

        let reverse_suffix_array = construct_suffix_array_naive(&reverse_reference);

        let params = ApproxSearchParams {
            reference: &reference,
            query: &string_to_ints("ATT"),
            o_table: &generate_o_table(&reference, &suffix_array),
            c_table: &generate_c_table(&reference),
            o_rev_table: &generate_o_table(&reverse_reference, &reverse_suffix_array),
            edits: 1,
        };

        let search_result = approx_search(params);

        println!("Actual results: ");

        for res in search_result.iter() {
            let mut bwt_string = String::new();
            for i in res.1..(res.1 + reference.len()) {
                let i = i % reference.len();
                let bwt_int = bwt(&reference, &suffix_array, i);
                let bwt_char = ALPHABET[bwt_int as usize];
                bwt_string.push(bwt_char);
            }

            println!("{:?} for bwt> {:?}", res, bwm(&reference, res.0));
        }

        assert_eq!(search_result.len(), 5);
        assert!(search_result.contains(&(13, 14, "IMM".to_string(), 1)));
        assert!(search_result.contains(&(5, 7, "MMI".to_string(), 1)));
        assert!(search_result.contains(&(5, 7, "MIM".to_string(), 1)));
        assert!(search_result.contains(&(6, 7, "MMM".to_string(), 0)));
        assert!(search_result.contains(&(5, 6, "MMS".to_string(), 1)));
    }

    #[test]
    fn test_att_with_0_edit() {
        let reference = string_to_ints("AGATAGATTCACA$");
        let suffix_array = construct_suffix_array_naive(&reference);

        let reverse_reference: Vec<u8> = reference.iter().rev().map(|&x| x).collect();

        let reverse_suffix_array = construct_suffix_array_naive(&reverse_reference);

        let params = ApproxSearchParams {
            reference: &reference,
            query: &string_to_ints("ATT"),
            o_table: &generate_o_table(&reference, &suffix_array),
            c_table: &generate_c_table(&reference),
            o_rev_table: &generate_o_table(&reverse_reference, &reverse_suffix_array),
            edits: 0,
        };

        let search_result = approx_search(params);
        println!("Actual result: {:?}", search_result);

        assert_eq!(search_result.len(), 1);
        assert!(search_result.contains(&(6, 7, String::from("MMM"), 0)));
    }

    #[test]
    fn test_simple() {
        let reference = string_to_ints("AAC$");
        let suffix_array = construct_suffix_array_naive(&reference);

        let reverse_reference: Vec<u8> = reference.iter().rev().map(|&x| x).collect();

        let reverse_suffix_array = construct_suffix_array_naive(&reverse_reference);

        let params = ApproxSearchParams {
            reference: &reference,
            query: &string_to_ints("ACC"),
            o_table: &generate_o_table(&reference, &suffix_array),
            c_table: &generate_c_table(&reference),
            o_rev_table: &generate_o_table(&reverse_reference, &reverse_suffix_array),
            edits: 1,
        };

        let search_result = approx_search(params);
        println!("Actual result: {:?}", search_result);

        assert_eq!(search_result.len(), 3);
        // assert!(search_result.contains(&(5, 5)));
        // assert!(search_result.contains(&(6, 6)));
    }

    #[test]
    fn test_substitute() {
        let reference = string_to_ints("ACG$");
        let suffix_array = construct_suffix_array_naive(&reference);

        let reverse_reference: Vec<u8> = reference.iter().rev().map(|&x| x).collect();

        let reverse_suffix_array = construct_suffix_array_naive(&reverse_reference);

        let params = ApproxSearchParams {
            reference: &reference,
            query: &string_to_ints("AGG"),
            o_table: &generate_o_table(&reference, &suffix_array),
            c_table: &generate_c_table(&reference),
            o_rev_table: &generate_o_table(&reverse_reference, &reverse_suffix_array),
            edits: 1,
        };

        let results = approx_search(params);
        println!("Actual result: {:?}", results);

        assert_eq!(results.len(), 1);
        assert!(results.contains(&(1, 2, "MSM".to_string(), 1)));
    }

    #[test]
    fn test_delete() {
        let reference = string_to_ints("TACGT$");
        let suffix_array = construct_suffix_array_naive(&reference);

        let reverse_reference: Vec<u8> = reference.iter().rev().map(|&x| x).collect();

        let reverse_suffix_array = construct_suffix_array_naive(&reverse_reference);

        let params = ApproxSearchParams {
            reference: &reference,
            query: &string_to_ints("TAGT"),
            o_table: &generate_o_table(&reference, &suffix_array),
            c_table: &generate_c_table(&reference),
            o_rev_table: &generate_o_table(&reverse_reference, &reverse_suffix_array),
            edits: 1,
        };

        let results = approx_search(params);
        println!("Actual result: {:?}", results);

        assert_eq!(results.len(), 1);
        match results.iter().next() {
            Some((start, end, edits, num_edits)) => {
                assert_eq!(start, &5);
                assert_eq!(end, &6);
                assert_eq!(edits, "MMDMM");
                assert_eq!(num_edits, &1);
            }
            None => {
                panic!("did not find any results")
            }
        }
    }

    #[test]
    fn test_insert() {
        let reference = string_to_ints("AC$");
        let suffix_array = construct_suffix_array_naive(&reference);

        let reverse_reference: Vec<u8> = reference.iter().rev().map(|&x| x).collect();

        let reverse_suffix_array = construct_suffix_array_naive(&reverse_reference);

        let params = ApproxSearchParams {
            reference: &reference,
            query: &string_to_ints("ACG"),
            o_table: &generate_o_table(&reference, &suffix_array),
            c_table: &generate_c_table(&reference),
            o_rev_table: &generate_o_table(&reverse_reference, &reverse_suffix_array),
            edits: 1,
        };

        let results = approx_search(params);
        println!("Actual result: {:?}", results);

        assert_eq!(results.len(), 1);
        match results.iter().next() {
            Some((start, end, edits, num_edits)) => {
                assert_eq!(start, &1);
                assert_eq!(end, &2);
                assert_eq!(edits, "MMI");
                assert_eq!(num_edits, &1);
            }
            None => {
                panic!("did not find any results")
            }
        }
    }

    #[test]
    fn test_too_long_query() {
        let reference = string_to_ints("ACGT$");
        let suffix_array = construct_suffix_array_naive(&reference);

        let reverse_reference: Vec<u8> = reference.iter().rev().map(|&x| x).collect();

        let reverse_suffix_array = construct_suffix_array_naive(&reverse_reference);

        let params = ApproxSearchParams {
            reference: &reference,
            query: &string_to_ints("ACGTGTGT"),
            o_table: &generate_o_table(&reference, &suffix_array),
            c_table: &generate_c_table(&reference),
            o_rev_table: &generate_o_table(&reverse_reference, &reverse_suffix_array),
            edits: 1,
        };

        let results = approx_search(params);
        println!("Actual result: {:?}", results);

        assert_eq!(results.len(), 0);
    }

    #[bench]
    fn bench_approx_search_ref1000(b: &mut Bencher) {
        let genome_string = read_genome(HG38_1000_PATH).unwrap();
        let genome = string_to_ints(&genome_string);
        let suffix_array = construct_suffix_array_naive(&genome);
        let reverse_reference: Vec<u8> = genome.iter().rev().map(|&x| x).collect();
        let reverse_suffix_array = construct_suffix_array_naive(&reverse_reference);
        let params = ApproxSearchParams {
            reference: &genome,
            query: &string_to_ints("CTCCATCATGTCTTATGGCG"),
            o_table: &generate_o_table(&genome, &suffix_array),
            c_table: &generate_c_table(&genome),
            o_rev_table: &generate_o_table(&reverse_reference, &reverse_suffix_array),
            edits: 1,
        };
        b.iter(|| approx_search(params))
    }
}
