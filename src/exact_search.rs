use crate::types::SuffixArray;
use std::{cmp::{Ordering, min}, usize};

/// Burrows-Wheeler Transform based search
pub fn backwards_search_with_bwt(query: &[u8], o_table: &Vec<Vec<usize>>, c_table: &[usize]) -> (usize, usize) {
    
    let last_char = query
        .last()
        .expect("Got an empty string");

    let c = *last_char as usize;
    
    let mut start = c_table[c];
    
    let mut end: usize = if c+1 >= c_table.len(){
        o_table.len()-1
    } else {
        c_table[c+1]-1
    };

    for c in query[..query.len()-1].iter().rev() {
        let c = *c as usize;
        println!("(start: {}, end: {})",start,end);
        
        start = c_table[c] + o_table[start-1][c];
        end = c_table[c] + o_table[end][c]-1
    }   

    (start, end)
}

/// Exact search based on binary search through the suffix array
pub fn naive_exact_search(suffix_array: &SuffixArray, query: &[u8]) -> usize {
    let mut low = 0;
    let mut high = suffix_array.array.len();

    while low < high {
        let mid = low + (high - low) / 2;

        let i = suffix_array.array[mid];
        let j = min(i + query.len(), suffix_array.string.len());
        
        let cmp_string = &suffix_array.string[i..j];

        match query.cmp(&cmp_string) {
            Ordering::Less => high = mid - 1,
            Ordering::Greater => low = mid + 1,
            Ordering::Equal => return mid,
        }
    }

    low
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{string_to_ints, generate_c_table, generate_o_table, construct_suffix_array_naive};

    #[test]
    fn test_bwt_search_1_match() {
        let reference = string_to_ints("agatagattcaca$");
        let suffix_array = construct_suffix_array_naive(&reference);
        let o_table = generate_o_table(&suffix_array);
        let c_table = generate_c_table(&suffix_array);
        let search_string = string_to_ints("att");
        let search_result = backwards_search_with_bwt(&search_string, &o_table, &c_table);

        assert_eq!((6, 6), search_result);
    }

    #[test]
    fn test_bwt_search_2_matches() {
        let reference = string_to_ints("agaga$");
        let suffix_array = construct_suffix_array_naive(&reference);
        let o_table = generate_o_table(&suffix_array);
        let c_table = generate_c_table(&suffix_array);
        let search_string = string_to_ints("aga");
        let search_result = backwards_search_with_bwt(&search_string, &o_table, &c_table);

        assert_eq!((2, 3), search_result);
    }

    #[test]
    fn test_bwt_search_0_matches() {
        let reference = string_to_ints("agaga$");
        let suffix_array = construct_suffix_array_naive(&reference);
        let o_table = generate_o_table(&suffix_array);
        let c_table = generate_c_table(&suffix_array);
        let search_string = string_to_ints("aca");
        let search_result = backwards_search_with_bwt(&search_string, &o_table, &c_table);

        assert!(search_result.0 > search_result.1);
    }
}