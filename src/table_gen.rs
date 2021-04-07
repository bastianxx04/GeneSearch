use crate::types::OTable;
use crate::util::bwt;
use crate::ALPHABET;

/// Generate an O-table. Inspired by "String Algorithms in C"
pub fn generate_o_table<'a>(reference: &'a [u8], suffix_array: &'a [usize]) -> OTable<'a> {
    let mut o_table = OTable::new(&reference, &suffix_array);

    let (rows, cols) = o_table.shape();

    for a in 0..(rows as u8) {
        for i in 1..cols {
            if bwt(&reference, &suffix_array, i - 1) == a {
                o_table.set(a, i, o_table.get(a, i - 1) + 1);
            } else {
                o_table.set(a, i, o_table.get(a, i - 1));
            }
        }
    }

    o_table
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{construct_suffix_array_naive, read_genome, remap_string, HG38_1000_PATH};
    use test::Bencher;

    #[test]
    fn test_o_table_size() {
        let reference = remap_string("ACGTATCGTGACGGGCTATAGCGATGTCGATGC$");
        let sa = construct_suffix_array_naive(&reference);
        let o_table = generate_o_table(&reference, &sa);
        let (rows, cols) = o_table.shape();
        assert_eq!(rows, ALPHABET.len());
        assert_eq!(cols, reference.len() + 1);
    }

    #[bench]
    fn bench_o_table_ref1000(b: &mut Bencher) {
        let genome_string = read_genome(HG38_1000_PATH).unwrap();
        let genome = remap_string(&genome_string);
        let suffix_array = construct_suffix_array_naive(&genome);
        b.iter(|| generate_o_table(&genome, &suffix_array));
    }

    #[bench]
    fn bench_c_table_ref1000(b: &mut Bencher) {
        let genome_string = read_genome(HG38_1000_PATH).unwrap();
        let genome = remap_string(&genome_string);
        b.iter(|| generate_c_table(&genome));
    }
}
