use crate::ALPHABET;
use crate::SuffixArray;
use crate::util::{bwt, string_to_ints};
use crate::suffix_array_construction::construct_suffix_array_naive;

pub fn generate_o_table(suffix_array: &SuffixArray) -> Vec<Vec<usize>> {
    let rows = ALPHABET.len();
    let cols = suffix_array.string.len() + 1;

    let o_table = vec![vec![0; cols]; rows];

    for a in 0..rows {
        for i in 1..suffix_array.array.len() {
            
        }
    }

    let mut old_rank_table = vec![0, 0, 0, 0, 0];
    let mut new_bwt_output = Vec::new();
    for i in 0..suffix_array.array.len() {
        let mut new_rank_table = old_rank_table.clone();
        let j = bwt(suffix_array.string, &suffix_array.array, i);
        new_rank_table[usize::from(j)] += 1;
        old_rank_table = new_rank_table.clone();
        new_bwt_output.push(new_rank_table.clone());
    }
    new_bwt_output.push(old_rank_table);
    new_bwt_output
}

pub fn generate_o_table_naive(suffix_array: &SuffixArray) -> Vec<Vec<usize>> {
    //ORDER: A, C, G, T
    let mut old_rank_table = vec![0, 0, 0, 0, 0];
    let mut new_bwt_output = Vec::new();
    for i in 0..suffix_array.array.len() {
        let mut new_rank_table = old_rank_table.clone();
        let j = bwt(suffix_array.string, &suffix_array.array, i);
        new_rank_table[usize::from(j)] += 1;
        old_rank_table = new_rank_table.clone();
        new_bwt_output.push(new_rank_table.clone());
    }
    new_bwt_output.push(old_rank_table);
    new_bwt_output
}

pub fn generate_c_table(sa: &SuffixArray) -> Vec<usize> {    
    let mut counts = vec![0;5];
    for &c in sa.string {
        counts[usize::from(c)] += 1;
    }

    let mut c_table = vec![0];
    for i in 1..ALPHABET.len() {
        let v = c_table.last().unwrap() + counts[i - 1];
        c_table.push(v);
    }
    c_table
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_test_o_table(reference: &str) -> Vec<Vec<usize>> {
        let reference = string_to_ints(reference);
        let sa = construct_suffix_array_naive(&reference);
        
        generate_o_table_naive(&sa)
    }

    #[test]
    fn test_o_table_size() {
        let reference = "acgt$";
        let o_table = generate_test_o_table(reference);
        assert_eq!(o_table.len(), ALPHABET.len());
        assert_eq!(o_table[0].len(), reference.len() + 1);
    }
}