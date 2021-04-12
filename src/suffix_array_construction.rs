use crate::types::SuffixArray;
use crate::ALPHABET;

const UNDEFINED: usize = usize::MAX;

/// SA-IS
pub fn suffix_array_induced_sort(reference: &[u8]) -> SuffixArray {
    let reference: Vec<u32> = reference.iter().map(|&n| n as u32).collect();
    recursive_suff_arr_induce_sort(&reference, ALPHABET.len())
}

fn recursive_suff_arr_induce_sort(reference: &[u32], alphabet_size: usize) -> SuffixArray {
    let types = build_type_array(reference);
    let lms_array = build_lms_array(&types);
    let bucket_sizes = build_bucket_sizes(reference, alphabet_size);

    let mut suffix_array = vec![UNDEFINED; reference.len()];
    place_lms(reference, &mut suffix_array, &lms_array, &bucket_sizes);
    induce_l_types(reference, &mut suffix_array, &types, &bucket_sizes);
    induce_s_types(reference, &mut suffix_array, &types, &bucket_sizes);

    let (reduced_string, reduced_offsets, new_alphabet_size) =
        reduce_reference_string(reference, &suffix_array, &types);

    let reduced_sa = compute_reduced_suffix_array(&reduced_string, new_alphabet_size);

    // Clear suffix array
    suffix_array.fill(UNDEFINED);

    // Induce SA from SA1
    remap_lms(
        reference,
        &mut suffix_array,
        &reduced_string,
        &reduced_offsets,
        &reduced_sa,
        &bucket_sizes,
    );
    induce_l_types(reference, &mut suffix_array, &types, &bucket_sizes);
    induce_s_types(reference, &mut suffix_array, &types, &bucket_sizes);

    suffix_array
}

fn build_type_array(reference: &[u32]) -> Vec<bool> {
    let n = reference.len();
    let mut types = vec![false; n];
    types[n - 1] = true;

    for i in (0..n - 1).rev() {
        if reference[i] == reference[i + 1] {
            types[i] = types[i + 1];
        } else {
            types[i] = reference[i] < reference[i + 1];
        }
    }

    types
}

fn build_lms_array(types: &[bool]) -> Vec<usize> {
    let n = types.len();
    let mut lms_array = Vec::new();

    if types[0] {
        lms_array.push(0)
    }
    for i in 1..n {
        if types[i] && !types[i - 1] {
            lms_array.push(i)
        }
    }

    lms_array
}

fn build_bucket_sizes(reference: &[u32], alphabet_size: usize) -> Vec<usize> {
    let mut bucket_sizes = vec![0; alphabet_size];

    for &c in reference {
        bucket_sizes[c as usize] += 1;
    }

    bucket_sizes
}

pub fn find_bucket_heads(bucket_sizes: &[usize]) -> Vec<usize> {
    let mut heads = vec![0; bucket_sizes.len()];

    for i in 1..bucket_sizes.len() {
        heads[i] = heads[i - 1] + bucket_sizes[i - 1];
    }

    heads
}

pub fn find_bucket_tails(bucket_sizes: &[usize]) -> Vec<usize> {
    let mut tails = vec![0; bucket_sizes.len()];
    tails[0] = bucket_sizes[0];

    for i in 1..bucket_sizes.len() {
        tails[i] = tails[i - 1] + bucket_sizes[i];
    }

    tails
}

fn place_lms(
    reference: &[u32],
    suffix_array: &mut SuffixArray,
    lms_array: &[usize],
    bucket_sizes: &[usize],
) {
    let mut bucket_tails = find_bucket_tails(bucket_sizes);

    for &i in lms_array {
        let c = reference[i];
        bucket_tails[c as usize] -= 1;
        suffix_array[bucket_tails[c as usize]] = i;
    }
}

fn induce_l_types(
    reference: &[u32],
    suffix_array: &mut SuffixArray,
    types: &[bool],
    bucket_sizes: &[usize],
) {
    let mut bucket_heads = find_bucket_heads(bucket_sizes);

    for i in 0..reference.len() {
        if suffix_array[i] == usize::MAX || suffix_array[i] == 0 {
            continue;
        }

        let j = suffix_array[i] - 1;

        if !types[j] {
            let c = reference[j] as usize;
            let bucket_index = bucket_heads[c];
            suffix_array[bucket_index] = j;
            bucket_heads[c] += 1;
        }
    }
}

fn induce_s_types(
    reference: &[u32],
    suffix_array: &mut SuffixArray,
    types: &[bool],
    bucket_sizes: &[usize],
) {
    let mut bucket_tails = find_bucket_tails(bucket_sizes);

    for i in (0..reference.len()).rev() {
        if suffix_array[i] == usize::MAX || suffix_array[i] == 0 {
            continue;
        }

        let j = suffix_array[i] - 1;
        if types[j] {
            let c = reference[j] as usize;
            bucket_tails[c] -= 1;
            suffix_array[bucket_tails[c]] = j;
        }
    }
}

fn reduce_reference_string(
    reference: &[u32],
    suffix_array: &[usize],
    types: &[bool],
) -> (Vec<u32>, Vec<usize>, usize) {
    let n = reference.len();
    let mut names_buf = vec![usize::MAX; n + 1];
    let mut name = 0;

    names_buf[suffix_array[0]] = name;
    let mut prev_lms_substring = suffix_array[0];

    for &j in suffix_array.iter() {
        if !is_lms_index(types, j) {
            continue;
        }

        if !compare_lms(reference, types, prev_lms_substring, j) {
            name += 1;
        }
        prev_lms_substring = j;
        names_buf[j] = name;
    }

    let new_alphabet_size = name + 1;

    let mut reduced_string = Vec::new();
    let mut reduced_offsets = Vec::new();
    for (i, &name) in names_buf.iter().enumerate() {
        if name == UNDEFINED {
            continue;
        }
        reduced_offsets.push(i);
        reduced_string.push(name as u32);
    }

    (reduced_string, reduced_offsets, new_alphabet_size)
}

fn is_lms_index(types: &[bool], j: usize) -> bool {
    if j == 0 {
        false
    } else {
        types[j] && !types[j - 1]
    }
}

fn compare_lms(reference: &[u32], types: &[bool], i: usize, j: usize) -> bool {
    if i == j {
        return true;
    }

    let n = reference.len();
    if i == n - 1 || j == n - 1 {
        return false;
    }

    let mut k = 0;
    loop {
        if k > 0 && is_lms_index(types, i + k) && is_lms_index(types, j + k) {
            return true;
        }

        if reference[i + k] != reference[j + k] || types[i + k] != types[j + k] {
            return false;
        }

        k += 1;
    }
}

/// Computes the suffix array for the reduced string.
/// If each character in the reduced string is unique, it is computed directly.
/// Otherwise, it is computed using the SA-IS algorithm recursively.
fn compute_reduced_suffix_array(reduced_string: &[u32], alphabet_size: usize) -> Vec<usize> {
    if reduced_string.len() == alphabet_size { //TODO: should this be equal? it might be <= or something like that
        // There are no duplicates in the reduced string
        let mut reduced_sa = vec![0; reduced_string.len()];
        reduced_sa[0] = alphabet_size;
        for (i, item) in reduced_string.iter().enumerate() {
            let j = *item as usize;
            reduced_sa[j] = i;
        }

        reduced_sa
    } else {
        // There are duplicates in the reduced string
        recursive_suff_arr_induce_sort(reduced_string, alphabet_size)
    }
}

fn remap_lms(
    reference: &[u32],
    suffix_array: &mut [usize],
    reduced_string: &[u32],
    reduced_offsets: &[usize],
    reduced_sa: &[usize],
    bucket_sizes: &[usize],
) {
    let mut bucket_tails = find_bucket_tails(bucket_sizes);
    for i in (1..(reduced_string.len() + 1)).rev() {
        let j = reduced_offsets[reduced_sa[i - 1]];
        let c = reference[j] as usize;
        bucket_tails[c] -= 1;
        suffix_array[bucket_tails[c]] = j;
    }
}

/// Construct a suffix array naively
pub fn construct_suffix_array_naive(reference: &[u8]) -> SuffixArray {
    let mut temp_data_table: Vec<(Vec<u8>, usize)> = Vec::new();
    for i in 0..(reference.len()) {
        let mut to_be_inserted = reference.to_owned();

        if !temp_data_table.is_empty() {
            to_be_inserted = temp_data_table.last().unwrap().0.clone();
            to_be_inserted.remove(0);
        }
        temp_data_table.push((to_be_inserted, i));
    }

    temp_data_table.sort();

    temp_data_table.iter().map(|elem| elem.1).collect()
}

#[cfg(test)]
mod tests {
    use crate::{read_genome, util::remap_string, HG38_1000000_PATH, HG38_1000_PATH};

    use super::*;
    use test::Bencher;

    const GEN60: &str = "AATAAACCTTACCTAGCACTCCATCATGTCTTATGGCGCGTGATTTGCCCCGGACTCAGG$";

    #[test]
    fn test_type_map() {
        let reference = remap_string("ACATGA$");
        let types = build_type_array(&reference);
        assert_eq!(vec![true, false, true, false, false, false, true], types);
    }

    #[test]
    fn test_sais_mmiissiissiippii() {
        let reference = remap_string("CCAATTAATTAAGGAA$");
        let sa = suffix_array_induced_sort(&reference);
        assert_eq!(
            vec![16, 15, 14, 10, 6, 2, 11, 7, 3, 1, 0, 13, 12, 9, 5, 8, 4],
            sa
        );
    }

    #[test]
    fn test_sais_aaa() {
        let reference = remap_string("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA$");
        let sa = suffix_array_induced_sort(&reference);
        println!("{:?}",sa);
        assert_eq!(
            320,
            sa.len()
        );
    }

    #[test]
    fn test_sais_cacag() {
        // CACAG isn't instantly turned into a suffix array by the first induced sort
        let reference = remap_string("CACAG$");
        let sa = suffix_array_induced_sort(&reference);
        assert_eq!(vec![5, 1, 3, 0, 2, 4], sa);
    }

    #[test]
    fn test_sais_compare_naive_mmiissiissiippii() {
        let genome = remap_string("CCAATTAATTAAGGAA$");
        let naive = construct_suffix_array_naive(&genome);
        let sais = suffix_array_induced_sort(&genome);
        for i in 0..sais.len() {
            if sais[i] != naive[i] {
                let print = &genome[sais[i]..];
                println!("{}: {:?}, ", i, print);
            }
        }
        assert_eq!(naive, sais);
    }

    #[test]
    fn test_sais_compare_naive() {
        let genome = remap_string(GEN60);
        let naive = construct_suffix_array_naive(&genome);
        let sais = suffix_array_induced_sort(&genome);
        for i in 0..sais.len() {
            if sais[i] != naive[i] {
                let print = &genome[sais[i]..];
                println!("{}: {:?}, ", i, print);
            }
        }
        assert_eq!(naive, sais);
    }

    #[bench]
    fn bench_sais_ref1000(b: &mut Bencher) {
        let genome_string = read_genome(HG38_1000_PATH).unwrap();
        let genome = remap_string(&genome_string);
        b.iter(|| suffix_array_induced_sort(&genome))
    }

    #[bench]
    fn bench_sais_ref1000000(b: &mut Bencher) {
        let genome_string = read_genome(HG38_1000000_PATH).unwrap();
        let genome = remap_string(&genome_string);
        b.iter(|| suffix_array_induced_sort(&genome))
    }

    #[bench]
    #[ignore = "very very slow"]
    fn bench_naive_ref1000000(b: &mut Bencher) {
        let genome_string = read_genome(HG38_1000000_PATH).unwrap();
        let genome = remap_string(&genome_string);
        b.iter(|| construct_suffix_array_naive(&genome))
    }
}
