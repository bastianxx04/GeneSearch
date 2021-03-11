use crate::types::SuffixArray;
use std::cmp::{Ordering, min};
use crate::ALPHABET;

/// Burrows-Wheeler Transform based search
pub fn exact_search(query: &[char], o_table: &Vec<Vec<usize>>, c_table: &[usize]) -> (usize, usize) {
    
    let last_char = query
        .last()
        .expect("Got an empty string");
    let last_char_num = ALPHABET.iter()
        .position(|&r| r == *last_char)
        .expect("Last char wasn't in alphabet");

    let mut start = c_table[last_char_num];
    let mut end = c_table[(last_char_num + 1) % ALPHABET.len()]-1;

    for c in query[..query.len()-1].iter().rev() {
        let char_num = ALPHABET.iter()
            .position(|&r| r == *c)
            .expect("Query char wasn't in alphabet");

        println!("(start: {}, end: {})",start,end);
        
        start = c_table[char_num] + o_table[start-1][char_num];
        end = c_table[char_num] + o_table[end][char_num]-1
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