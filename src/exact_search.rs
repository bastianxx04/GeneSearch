use crate::types::{OTable, SuffixArray};
use std::{
    cmp::{min, Ordering},
    usize,
};

/// Burrows-Wheeler Transform based search
pub fn backwards_search_with_bwt(
    query: &[u8],
    o_table: &OTable,
    c_table: &[usize],
) -> (usize, usize) {
    println!("{:?}", c_table);
    println!("{}", o_table);
    let c = *query.last().expect("Got an empty string") as usize;

    let mut start = c_table[c];

    let mut end: usize = if c + 1 < c_table.len() {
        c_table[c + 1]
    } else {
        o_table.shape().1 - 1
    };

    for &c in query[..query.len() - 1].iter().rev() {
        println!("(start: {}, end: {})", start, end);
        let c_rank = c_table[c as usize] - 1; // c_rank = 4
        start = c_rank + o_table.get(c, start) + 1;
        end = c_rank + o_table.get(c, end);
    }

    (start, end - 1)
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
    use crate::{construct_suffix_array_naive, generate_c_table, generate_o_table, string_to_ints};

    #[test]
    fn test_bwt_search_1_match() {
        let reference = string_to_ints("cattga$");
        let suffix_array = construct_suffix_array_naive(&reference);
        let o_table = generate_o_table(&suffix_array);
        let c_table = generate_c_table(&suffix_array);
        let search_string = string_to_ints("att");
        let search_result = backwards_search_with_bwt(&search_string, &o_table, &c_table);

        assert_eq!((2, 2), search_result);
    }

    #[test]
    fn test_bwt_search_banana() {
        let reference = string_to_ints("cagaga$");
        let suffix_array = construct_suffix_array_naive(&reference);
        let o_table = generate_o_table(&suffix_array);
        let c_table = generate_c_table(&suffix_array);
        let search_string = string_to_ints("aga");
        let search_result = backwards_search_with_bwt(&search_string, &o_table, &c_table);

        assert_eq!((2, 3), search_result);
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
