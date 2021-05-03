use crate::util::bwt;
use crate::ALPHABET;

#[allow(dead_code)]
fn generate_o_table_naive(reference: &[u8], suffix_array: &[usize]) -> Vec<Vec<usize>> {
    //ORDER: A, C, G, T
    let mut old_rank_table = vec![0, 0, 0, 0, 0];
    let mut new_bwt_output = Vec::new();
    for i in 0..suffix_array.len() {
        let mut new_rank_table = old_rank_table.clone();
        let j = bwt(reference, &suffix_array, i);
        new_rank_table[usize::from(j)] += 1;
        old_rank_table = new_rank_table.clone();
        new_bwt_output.push(new_rank_table.clone());
    }
    new_bwt_output.push(old_rank_table);
    new_bwt_output
}

pub fn generate_c_table(reference: &[u8]) -> Vec<usize> {
    let mut counts = vec![0; 5];
    for &c in reference {
        counts[usize::from(c)] += 1;
    }

    let mut c_table = vec![0];
    for i in 1..ALPHABET.len() {
        let v = c_table.last().unwrap() + counts[i - 1];
        c_table.push(v);
    }
    c_table
}
