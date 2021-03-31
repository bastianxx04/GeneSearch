use crate::types::SuffixArray;
use std::usize;

/// SA-IS
#[allow(dead_code)]
pub fn suffix_array_induced_sort(reference: &Vec<usize>, alphabet_size: usize) -> SuffixArray {
    let n = reference.len();
    let t = build_type_array(reference);
    let lms_pointers = build_lms_array(&t);

    let bucket_sizes = build_bucket_sizes(reference, alphabet_size);
    let bucket_heads = find_bucket_heads(&bucket_sizes);
    let bucket_tails = find_bucket_tails(&bucket_sizes);

    let mut suffix_array = vec![usize::MAX; n];
    place_lms(
        &mut suffix_array,
        reference,
        &lms_pointers,
        bucket_tails.clone(),
    );
    induce_l_types(&mut suffix_array, reference, &t, bucket_heads.clone());
    induce_s_types(&mut suffix_array, reference, &t, bucket_tails.clone());

    let (reduced_string, new_alphabet_len) =
        reduce_reference_string(reference, &suffix_array, &lms_pointers, &t);

    let mut buckets = Vec::new();
    let mut sa1 = Vec::new();
    for i in lms_pointers {
        let bucket = find_bucket(&bucket_tails, i);
        if buckets.contains(&bucket) {
            //duplicate found
            sa1 = suffix_array_induced_sort(&reduced_string, new_alphabet_len);
        } else {
            buckets.push(bucket);
        }
    }

    // TODO: Induce SA from SA1
    println!("sa1: {:?}", sa1);
    suffix_array
}

fn find_bucket(bucket_tails: &Vec<usize>, i: usize) -> usize {
    match bucket_tails.iter().position(|&t| i <= t) {
        Some(v) => v,
        None => panic!("index out of bucket"),
    }
}

fn build_type_array(reference: &Vec<usize>) -> Vec<bool> {
    let n = reference.len();
    let mut type_map = vec![false; n];
    type_map[n - 1] = true;

    for i in (0..n - 1).rev() {
        if reference[i] == reference[i + 1] {
            type_map[i] = type_map[i + 1];
        } else {
            type_map[i] = reference[i] < reference[i + 1];
        }
    }

    type_map
}

fn build_lms_array(t: &[bool]) -> Vec<usize> {
    let n = t.len();
    let mut lms_substrings = Vec::new();

    if t[0] {
        lms_substrings.push(0)
    }
    for i in 1..n {
        if t[i] && !t[i - 1] {
            lms_substrings.push(i)
        }
    }

    lms_substrings
}

fn build_bucket_sizes(reference: &[usize], alphabet_size: usize) -> Vec<usize> {
    let mut bucket_sizes = vec![0; alphabet_size];

    for &c in reference {
        bucket_sizes[c as usize] += 1;
    }

    bucket_sizes
}

fn find_bucket_heads(buckets: &[usize]) -> Vec<usize> {
    let mut offset = 1;
    let mut result = Vec::new();

    for size in buckets {
        result.push(offset);
        offset += size;
    }

    result
}

fn find_bucket_tails(buckets: &[usize]) -> Vec<usize> {
    let mut offset = 1;
    let mut result = Vec::new();

    for size in buckets {
        offset += size;
        result.push(offset - 1);
    }

    result
}

fn place_lms(
    suffix_array: &mut SuffixArray,
    reference: &[usize],
    lms_pointers: &[usize],
    mut bucket_tails: Vec<usize>,
) {
    for &i in lms_pointers {
        let c = reference[i];
        suffix_array[bucket_tails[c] - 1] = i;
        bucket_tails[c] -= 1;
    }
}

fn induce_l_types(
    suffix_array: &mut SuffixArray,
    reference: &[usize],
    t: &[bool],
    mut bucket_heads: Vec<usize>,
) {
    // STEP 2 (it's about to get crazy)
    let n = reference.len();
    for i in 0..n {
        if suffix_array[i] == usize::MAX || suffix_array[i] == 0 {
            continue;
        }

        let j = suffix_array[i] - 1;

        if !t[j] {
            let c = reference[j] as usize;
            suffix_array[bucket_heads[c] - 1] = j;
            bucket_heads[c] += 1;
        }
    }
}

fn induce_s_types(
    suffix_array: &mut SuffixArray,
    reference: &[usize],
    t: &[bool],
    mut bucket_tails: Vec<usize>,
) {
    // STEP 3 (the one where the magic happens)
    let n = reference.len();
    for i in (0..n).rev() {
        if suffix_array[i] == usize::MAX || suffix_array[i] == 0 {
            continue;
        }

        let j = suffix_array[i] - 1;
        if t[j] {
            let c = reference[j] as usize;
            suffix_array[bucket_tails[c] - 1] = j;
            bucket_tails[c] -= 1;
        }
    }
}

fn reduce_reference_string(
    reference: &[usize],
    suffix_array: &SuffixArray,
    lms_pointers: &[usize],
    t: &[bool],
) -> (Vec<usize>, usize) {
    let n = reference.len();
    let mut names_buf = vec![usize::MAX; n + 1];
    let mut name = 0;
    let mut reduced_string = Vec::new();

    names_buf[suffix_array[0]] = name;
    let mut prev_lms_substring = suffix_array[0];

    for i in 1..(n) {
        let j = suffix_array[i];

        if !lms_pointers.contains(&j) {
            continue;
        }

        if !compare_lms(reference, t, lms_pointers, prev_lms_substring, j) {
            name += 1;
        }
        prev_lms_substring = j;
        names_buf[j] = name;
    }

    let new_alphabet_size = name + 1;

    for i in 0..(n + 1) {
        name = names_buf[i];
        if name == usize::MAX {
            continue;
        }
        reduced_string.push(name);
    }

    println!("reduced ref string: {:?}", reduced_string);
    (reduced_string, new_alphabet_size)
}

fn compare_lms(
    reference: &[usize],
    t: &[bool],
    lms_pointers: &[usize],
    i: usize,
    j: usize,
) -> bool {
    if i == j {
        return true;
    }

    let n = reference.len();
    if i == n - 1 || j == n - 1 {
        return false;
    }

    let mut k = 0;
    loop {
        if k > 0 && lms_pointers.contains(&(i + k)) && lms_pointers.contains(&(j + k)) {
            return true;
        }

        if reference[i + k] != reference[j + k] || t[i + k] != t[j + k] {
            return false;
        }

        k += 1;
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
    use crate::{read_genome, util::remap_string, ALPHABET, HG38_1000_PATH};

    use super::*;
    use test::Bencher;

    const GEN60: &str = "AATAAACCTTACCTAGCACTCCATCATGTCTTATGGCGCGTGATTTGCCCCGGACTCAGG$";

    #[test]
    fn test_type_map() {
        let reference = remap_string("ACATGA$");
        let t = build_type_array(&reference);
        assert_eq!(vec![true, false, true, false, false, false, true], t);
    }

    #[test]
    fn test_sais_mmiissiissiippii() {
        let reference = remap_string("CCAATTAATTAAGGAA$");
        let sa = suffix_array_induced_sort(&reference, ALPHABET.len());
        assert_eq!(
            vec![16, 15, 14, 10, 6, 2, 11, 7, 3, 1, 0, 13, 12, 9, 5, 8, 4],
            sa
        );
    }

    #[test]
    fn test_sais_compare_naive() {
        let genome_u8 = remap_string(GEN60);
        let naive = construct_suffix_array_naive(&genome_u8);
        let genome_usize = genome_u8.iter().map(|&c| c as usize).collect();
        let sais = suffix_array_induced_sort(&genome_usize, ALPHABET.len());
        for i in 0..sais.len() {
            if sais[i] != naive[i] {
                let print = &genome_usize[sais[i]..];
                println!("{}: {:?}, ", i, print);
            }
        }
        assert_eq!(naive, sais);
    }

    #[bench]
    #[ignore = "slow"]
    fn bench_sa_naive_ref1000(b: &mut Bencher) {
        let genome_string = read_genome(HG38_1000_PATH).unwrap();
        let genome = remap_string(&genome_string);
        b.iter(|| construct_suffix_array_naive(&genome))
    }

    #[bench]
    #[ignore = "slow"]
    fn bench_sais_ref1000(b: &mut Bencher) {
        let genome_string = read_genome(HG38_1000_PATH).unwrap();
        let genome = remap_string(&genome_string);
        b.iter(|| suffix_array_induced_sort(&genome, ALPHABET.len()))
    }
}
