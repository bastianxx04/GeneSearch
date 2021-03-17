use crate::DTable;
use crate::ALPHABET;
use crate::{types::OTable, CTable};
use std::collections::HashSet;
use std::convert::TryFrom;

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
    println!("D-table: {:?}", d_table);
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
                String::from("M"),
                0,
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
    c_table: &CTable,
    o_rev_table: &OTable,
) -> DTable {
    let mut start = 1;
    let mut end = reference.len() - 1;
    let mut edits_left = 0;
    let mut d_table: DTable = Vec::new();

    for i in 0..(query.len()) {
        let current_symbol = usize::from(query[i]);
        start = c_table[current_symbol] + o_rev_table.get(current_symbol as u8, start - 1) + 1;
        end = c_table[current_symbol] + o_rev_table.get(current_symbol as u8, end);
        if start > end {
            start = 1;
            end = reference.len() - 1;
            edits_left += 1;
        }
        d_table.push(edits_left);
    }

    return d_table;
}

fn inexact_recursion(
    query: &[u8],
    i: i32,
    edits_left: i32,
    left: usize,
    right: usize,
    d_table: &DTable,
    o_table: &OTable,
    c_table: &CTable,
    cigar: String,
    edits_total: usize,
) -> HashSet<(usize, usize, String, usize)> {
    // println!("entered recursive loop at level: {}", i);
    let lower_limit = match usize::try_from(i) {
        Ok(value) => d_table[value],
        Err(_) => 0,
    };

    if edits_left < lower_limit as i32 {
        // println!("  returned nothing");
        return HashSet::new();
    }

    let mut result_set = HashSet::new();

    if i < 0 {
        result_set.insert((left, right, cigar, edits_total));
        println!("  returned something: {:?}", result_set);
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
                format!("{}{}", "M", cigar),
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
    return result_set;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        bwt, construct_suffix_array_naive, generate_c_table, generate_o_table, string_to_ints,
    };

    #[test]
    fn test_att_with_1_edit() {
        let reference = string_to_ints("agatagattcaca$");
        let suffix_array = construct_suffix_array_naive(&reference);

        let reverse_reference: Vec<u8> = reference.iter().rev().map(|&x| x).collect();

        let reverse_suffix_array = construct_suffix_array_naive(&reverse_reference);

        let params = ApproxSearchParams {
            reference: &reference,
            query: &string_to_ints("att"),
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

            println!("{:?} for bwt> {}", res, bwt_string);
        }

        assert_eq!(search_result.len(), 2);
        // assert!(search_result.contains(&(5, 5)));
        // assert!(search_result.contains(&(6, 6)));
    }

    #[test]
    fn test_att_with_0_edit() {
        let reference = string_to_ints("agatagattcaca$");
        let suffix_array = construct_suffix_array_naive(&reference);

        let reverse_reference: Vec<u8> = reference.iter().rev().map(|&x| x).collect();

        let reverse_suffix_array = construct_suffix_array_naive(&reverse_reference);

        let params = ApproxSearchParams {
            reference: &reference,
            query: &string_to_ints("att"),
            o_table: &generate_o_table(&reference, &suffix_array),
            c_table: &generate_c_table(&reference),
            o_rev_table: &generate_o_table(&reverse_reference, &reverse_suffix_array),
            edits: 0,
        };

        let search_result = approx_search(params);
        println!("Actual result: {:?}", search_result);

        assert_eq!(search_result.len(), 2);
        // assert!(search_result.contains(&(5, 5)));
        // assert!(search_result.contains(&(6, 6)));
    }

    #[test]
    fn test_simple() {
        let reference = string_to_ints("aac$");
        let suffix_array = construct_suffix_array_naive(&reference);

        let reverse_reference: Vec<u8> = reference.iter().rev().map(|&x| x).collect();

        let reverse_suffix_array = construct_suffix_array_naive(&reverse_reference);

        let params = ApproxSearchParams {
            reference: &reference,
            query: &string_to_ints("acc"),
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
    fn test_delete() {
        let reference = string_to_ints("tacgt$");
        let suffix_array = construct_suffix_array_naive(&reference);

        let reverse_reference: Vec<u8> = reference.iter().rev().map(|&x| x).collect();

        let reverse_suffix_array = construct_suffix_array_naive(&reverse_reference);

        let params = ApproxSearchParams {
            reference: &reference,
            query: &string_to_ints("tagt"),
            o_table: &generate_o_table(&reference, &suffix_array),
            c_table: &generate_c_table(&reference),
            o_rev_table: &generate_o_table(&reverse_reference, &reverse_suffix_array),
            edits: 1,
        };

        let results = approx_search(params);
        println!("Actual result: {:?}", results);

        assert_eq!(results.len(), 1);
        match results.iter().next() {
            Some(v) => {
                assert_eq!(v.0, 5);
                assert_eq!(v.0, 5);
            }
            None => {}
        }
    }
}
