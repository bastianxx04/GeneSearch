use crate::ALPHABET;
use crate::SuffixArray;

pub fn generate_o_table(suffix_array: &SuffixArray) -> Vec<Vec<usize>> {
    //ORDER: A, C, G, T
    let mut old_rank_table = vec![0, 0, 0, 0, 0];
    let mut new_bwt_output = Vec::new();
    for i in 0..suffix_array.array.len() {
        let mut new_rank_table = old_rank_table.clone();
        let j = *prev_elem(suffix_array.string, suffix_array.array[i]);
        new_rank_table[usize::from(j)] += 1;
        old_rank_table = new_rank_table.clone();
        new_bwt_output.push(new_rank_table.clone());
    }
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

pub fn prev_elem<T>(array: &[T], i: usize) -> &T {
    &array[(array.len() + i - 1) % array.len()]
}
