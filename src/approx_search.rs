use std::collections::BTreeSet;
use crate::ALPHABET;
use crate::OTable;
use crate::CTable;
use crate::DTable;
use std::convert::TryFrom;

pub struct ApproxSearchParams<'a> {
    pub ref_string: &'a [u8],
    pub search_string: &'a [u8],
    pub o_table: &'a OTable,
    pub c_table: &'a CTable,
    pub o_table_rev: &'a OTable,
    pub edits_left: usize
}

/// Approximative search
pub fn approx_search(params: ApproxSearchParams) -> BTreeSet<(usize, usize)> {
    let d_table = calculate_d_table(params.ref_string, params.search_string, params.c_table, params.o_table_rev);
    inexact_recursion(params.search_string, (params.search_string.len() - 1) as i32, params.edits_left as i32, 1, params.ref_string.len() - 1, &d_table, params.o_table, params.c_table)
}

fn calculate_d_table(ref_word: &[u8], search_word: &[u8], c_table: &CTable, o_table: &OTable) -> DTable {
    let mut start = 1;
    println!("ref string: {:?}", ref_word);
    println!("search word: {:?}", search_word);
    let mut end = ref_word.len() - 1;
    let mut edits_left = 0;
    let mut D: DTable = Vec::new();

    for i in 0..(search_word.len()) {
        let i_char_num = usize::from(search_word[i]);
        start = c_table[i_char_num] + o_table[start - 1][i_char_num] + 1;
        end = c_table[i_char_num] + o_table[end][i_char_num];
        if start > end {
            start = 1;
            end = ref_word.len()-1;
            edits_left += 1;
        }
        D.push(edits_left);
    }
    println!("made D table with values: {:?}", D);
    return D
}

fn inexact_recursion(search_word: &[u8], i: i32, edits_left: i32, left: usize, right: usize, d_table: &DTable, o_table: &OTable, c_table: &CTable) -> BTreeSet<(usize, usize)>{
    // println!("entered recursive loop at level: {}", i);

    let mut left = left;
    let mut right = right;
    
    let lower_limit = match usize::try_from(i) {
        Ok(value) => d_table[value], 
        Err(_) => 1,
    };
    
    if edits_left < lower_limit as i32 {
        // println!("  returned nothing");
        return BTreeSet::new()
    }

    if i < 0 {
        let set: BTreeSet<(usize, usize)> = [(left, right)].iter().cloned().collect();
        // println!("  returned something: {:?}", set);
        return set
    }

    let mut result_set = BTreeSet::new();
    result_set = result_set.union(&inexact_recursion(search_word, i-1, edits_left-1, left, right, d_table, o_table, c_table)).cloned().collect();
    
    for (b, _) in ALPHABET.iter().enumerate() { 
        // println!("pre-math  start: {} - end: {}", left, right);
        left = c_table[b] + o_table[left][b];
        right = c_table[b] + o_table[right][b];
        // println!("post-math  start: {} - end: {}", left, right);
        if left <= right {
            result_set = result_set.union(&inexact_recursion(search_word, i, edits_left-1, left, right, d_table, o_table, c_table)).cloned().collect();
            if b == search_word[i as usize].into() {
                result_set = result_set.union(&inexact_recursion(search_word, i-1, edits_left, left, right, d_table, o_table, c_table)).cloned().collect();
            } else {
                result_set = result_set.union(&inexact_recursion(search_word, i-1, edits_left-1, left, right, d_table, o_table, c_table)).cloned().collect();
            }
        }
    }
    return result_set
} 

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{string_to_ints, generate_c_table, generate_o_table, construct_suffix_array_naive};

    #[test]
    fn test_1() {
        let reference = string_to_ints("agatagattcaca$");
        let suffix_array = construct_suffix_array_naive(&reference);

        let mut reverse_reference = reference.clone();
        reverse_reference.reverse();

        let reverse_suffix_array = construct_suffix_array_naive(&reverse_reference);

        let params = ApproxSearchParams {
            ref_string: &reference, 
            search_string: &string_to_ints("att"),
            o_table: &generate_o_table(&suffix_array),
            c_table: &generate_c_table(&suffix_array),
            o_table_rev: &generate_o_table(&reverse_suffix_array),
            edits_left: 1,
        };

        let search_result = approx_search(params);
        println!("Actual result: {:?}", search_result);

        assert_eq!(search_result.len(), 2);
        assert!(search_result.contains(&(5, 5)));
        assert!(search_result.contains(&(6, 6)));
    }
}