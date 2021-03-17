use crate::DTable;
use crate::ALPHABET;
use crate::{types::OTable, CTable};
use std::collections::HashSet;
use std::convert::TryFrom;

pub struct ApproxSearchParams<'a> {
    pub ref_string: &'a [u8],
    pub search_string: &'a [u8],
    pub o_table: &'a OTable<'a>,
    pub c_table: &'a CTable,
    pub o_table_rev: &'a OTable<'a>,
    pub edits_left: usize,
}

/// Approximative search
pub fn approx_search(params: ApproxSearchParams) -> HashSet<(usize, usize, String, usize)> {
    //let d_table = calculate_d_table(params.ref_string, params.search_string, params.c_table, params.o_table_rev);

    let d_table = vec![0; params.search_string.len()];

    inexact_recursion(
        params.search_string,
        (params.search_string.len() - 1) as i32,
        params.edits_left as i32,
        1,
        params.ref_string.len() - 1,
        &d_table,
        params.o_table,
        params.c_table,
        "".to_string(),
        0,
    )
}

#[allow(dead_code)]
fn calculate_d_table(
    ref_word: &[u8],
    search_string: &[u8],
    c_table: &CTable,
    o_table: &Vec<Vec<usize>>,
) -> DTable {
    let mut start = 1;
    println!("ref string: {:?}", ref_word);
    println!("search word: {:?}", search_string);
    let mut end = ref_word.len() - 1;
    let mut edits_left = 0;
    let mut d_table: DTable = Vec::new();

    for i in 0..(search_string.len()) {
        let i_char_num = usize::from(search_string[i]);
        start = c_table[i_char_num] + o_table[start - 1][i_char_num] + 1;
        end = c_table[i_char_num] + o_table[end][i_char_num];
        if start > end {
            start = 1;
            end = ref_word.len() - 1;
            edits_left += 1;
        }
        d_table.push(edits_left);
    }
    println!("made D table with values: {:?}", d_table);
    return d_table;
}

fn inexact_recursion(
    search_string: &[u8],
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
    let current_char = search_string[i as usize];
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
                search_string,
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
            search_string,
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
                search_string,
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
            ref_string: &reference,
            search_string: &string_to_ints("att"),
            o_table: &generate_o_table(&reference, &suffix_array),
            c_table: &generate_c_table(&reference),
            o_table_rev: &generate_o_table(&reverse_reference, &reverse_suffix_array),
            edits_left: 1,
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
            ref_string: &reference,
            search_string: &string_to_ints("att"),
            o_table: &generate_o_table(&reference, &suffix_array),
            c_table: &generate_c_table(&reference),
            o_table_rev: &generate_o_table(&reverse_reference, &reverse_suffix_array),
            edits_left: 0,
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
            ref_string: &reference,
            search_string: &string_to_ints("acc"),
            o_table: &generate_o_table(&reference, &suffix_array),
            c_table: &generate_c_table(&reference),
            o_table_rev: &generate_o_table(&reverse_reference, &reverse_suffix_array),
            edits_left: 1,
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
            ref_string: &reference,
            search_string: &string_to_ints("tagt"),
            o_table: &generate_o_table(&reference, &suffix_array),
            c_table: &generate_c_table(&reference),
            o_table_rev: &generate_o_table(&reverse_reference, &reverse_suffix_array),
            edits_left: 1,
        };

        let search_result = approx_search(params);
        println!("Actual result: {:?}", search_result);

        assert_eq!(search_result.len(), 1);
        //assert!(search_result.contains(&(5, 5)));
    }
}
