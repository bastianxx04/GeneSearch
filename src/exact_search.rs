use crate::types::OTable;
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
        construct_suffix_array_naive, generate_c_table, generate_o_table, read_genome,
        remap_string, HG38_1000_PATH,
    };
    use test::Bencher;

    #[test]
    fn test_bwt_search_1_match() {
        let reference = remap_string("CATTGA$");
        let suffix_array = construct_suffix_array_naive(&reference);
        let o_table = generate_o_table(&reference, &suffix_array);
        let c_table = generate_c_table(&reference);
        let search_string = remap_string("ATT");
        let search_result = bwt_search(&search_string, &o_table, &c_table);

        assert_eq!((2, 2), search_result);
    }

    #[test]
    fn test_bwt_search_banana() {
        let reference = remap_string("CAGAGA$");
        let suffix_array = construct_suffix_array_naive(&reference);
        let o_table = generate_o_table(&reference, &suffix_array);
        let c_table = generate_c_table(&reference);
        let search_string = remap_string("AGA");
        let search_result = bwt_search(&search_string, &o_table, &c_table);

        assert_eq!((2, 3), search_result);
    }

    #[test]
    fn test_bwt_search_2_matches() {
        let reference = remap_string("AGAGA$");
        let suffix_array = construct_suffix_array_naive(&reference);
        let o_table = generate_o_table(&reference, &suffix_array);
        let c_table = generate_c_table(&reference);
        let search_string = remap_string("AGA");
        let search_result = bwt_search(&search_string, &o_table, &c_table);

        assert_eq!((2, 3), search_result);
    }

    #[test]
    fn test_bwt_search_0_matches() {
        let reference = remap_string("AGAGA$");
        let suffix_array = construct_suffix_array_naive(&reference);
        let o_table = generate_o_table(&reference, &suffix_array);
        let c_table = generate_c_table(&reference);
        let search_string = remap_string("ACA");
        let search_result = bwt_search(&search_string, &o_table, &c_table);

        assert!(search_result.0 > search_result.1);
    }

    #[test]
    fn test_bwt_search_query_longer_than_reference() {
        let reference = remap_string("AGAGA$");
        let suffix_array = construct_suffix_array_naive(&reference);
        let o_table = generate_o_table(&reference, &suffix_array);
        let c_table = generate_c_table(&reference);
        let search_string = remap_string("ACAAGAGAGA");
        let search_result = bwt_search(&search_string, &o_table, &c_table);

        assert!(search_result.0 > search_result.1);
    }

    #[bench]
    fn bench_bwt_search_ref1000_query20(b: &mut Bencher) {
        let genome_string = read_genome(HG38_1000_PATH).unwrap();
        let genome = remap_string(&genome_string);
        let suffix_array = construct_suffix_array_naive(&genome);
        let o_table = generate_o_table(&genome, &suffix_array);
        let c_table = generate_c_table(&genome);
        let query = remap_string("CTCCATCATGTCTTATGGCG");
        b.iter(|| bwt_search(&query, &o_table, &c_table));
    }
}
