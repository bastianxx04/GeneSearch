use crate::o_table::OTable;
use std::{
    cmp::{min, Ordering},
    usize,
};

/// Backwards search with Burrows-Wheeler Transform. Inspired by "String Algorithms in C".
pub fn bwt_search(query: &[u8], o_table: &OTable, c_table: &[usize]) -> (usize, usize) {
    let (_, cols) = o_table.shape();

    let mut start = 0;
    let mut end = cols - 1;

    for &a in query.iter().rev() {
        start = c_table[a as usize] + o_table.get(a, start);
        end = c_table[a as usize] + o_table.get(a, end);
    }

    (start, end - 1)
}

/// Exact search based on binary search through the suffix array
#[allow(dead_code)]
pub fn naive_exact_search(reference: &[u8], suffix_array: &[usize], query: &[u8]) -> usize {
    let mut low = 0;
    let mut high = suffix_array.len();

    while low < high {
        let mid = low + (high - low) / 2;

        let i = suffix_array[mid];
        let j = min(i + query.len(), reference.len());

        let cmp_string = &reference[i..j];

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
    use crate::{
        suffix_array_construction::construct_suffix_array_naive,
        table_gen::generate_c_table,
        util::{remap_query, remap_reference},
    };

    #[test]
    fn test_bwt_search_1_match() {
        let reference = remap_reference("CATTGA");
        let suffix_array = construct_suffix_array_naive(&reference);
        let o_table = OTable::new(&reference, &suffix_array);
        println!("{}", o_table);
        let c_table = generate_c_table(&reference);
        let query = remap_query("ATT");
        let search_result = bwt_search(&query, &o_table, &c_table);

        assert_eq!((2, 2), search_result);
    }

    #[test]
    fn test_bwt_search_banana() {
        let reference = remap_reference("CAGAGA");
        let suffix_array = construct_suffix_array_naive(&reference);
        let o_table = OTable::new(&reference, &suffix_array);
        let c_table = generate_c_table(&reference);
        let query = remap_query("AGA");
        let search_result = bwt_search(&query, &o_table, &c_table);

        assert_eq!((2, 3), search_result);
    }

    #[test]
    fn test_bwt_search_2_matches() {
        let reference = remap_reference("AGAGA");
        let suffix_array = construct_suffix_array_naive(&reference);
        let o_table = OTable::new(&reference, &suffix_array);
        let c_table = generate_c_table(&reference);
        let query = remap_query("AGA");
        let search_result = bwt_search(&query, &o_table, &c_table);

        assert_eq!((2, 3), search_result);
    }

    #[test]
    fn test_bwt_search_0_matches() {
        let reference = remap_reference("AGAGA");
        let suffix_array = construct_suffix_array_naive(&reference);
        let o_table = OTable::new(&reference, &suffix_array);
        let c_table = generate_c_table(&reference);
        let query = remap_query("ACA");
        let search_result = bwt_search(&query, &o_table, &c_table);

        assert!(search_result.0 > search_result.1);
    }

    #[test]
    fn test_bwt_search_query_longer_than_reference() {
        let reference = remap_reference("AGAGA");
        let suffix_array = construct_suffix_array_naive(&reference);
        let o_table = OTable::new(&reference, &suffix_array);
        let c_table = generate_c_table(&reference);
        let query = remap_query("ACAAGAGAGA");
        let search_result = bwt_search(&query, &o_table, &c_table);

        assert!(search_result.0 > search_result.1);
    }
}
