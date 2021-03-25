use std::usize;

use crate::types::SuffixArray;
use crate::ALPHABET;

/// SA-IS
#[allow(dead_code)]
pub fn suffix_array_induced_sort(reference: &[u8]) -> SuffixArray {
    /*
    t: array [0..n − 1] of boolean;
    P1: array [0..n_1 − 1] of integer;
    S1: array [0..n_1 − 1] of integer;
    B: array [0..∥Σ(S)∥ − 1] of integer;

    Scan S once to classify all the characters as L- or S-type into t;
    Scan t once to find all the LMS-substrings in S into P1;
    Induced sort all the LMS-substrings using P1 and B;
    Name each LMS-substring in S by its bucket index to get a new shortened string S1;
    if Each character in S1 is unique
        then
            Directly compute SA1 from S1;
        else
            SA-IS(S1, SA1); � where recursive call happens
    Induce SA from SA1;
    return
    */
    let n = reference.len();
    let t = build_type_array(reference);
    let lms_pointers = build_lms_array(&t);

    let bucket_sizes = build_bucket_sizes(reference);
    let bucket_heads = find_bucket_heads(&bucket_sizes);
    let bucket_tails = find_bucket_tails(&bucket_sizes);
    
    let mut suffix_array = vec![usize::MAX; n];
    place_lms(&mut suffix_array, reference, &lms_pointers, bucket_tails.clone());
    induce_l_types(&mut suffix_array, reference, &t, bucket_heads.clone());
    induce_s_types(&mut suffix_array, reference, &t, bucket_tails.clone());
    
    let reduced_string = reduce_reference_string(reference, &lms_pointers);
    // step 4: cry
    //let mut reduced_reference = Vec::new();
    // For hver LMS substring, find dens bucket index og læg ind i reduced_reference

    for i in lms_pointers {
        for j in 0..ALPHABET.len() {
            if i > bucket_tails[j] && i < bucket_heads[j] {
                
            }
        }
    }
    
    suffix_array
}

fn build_type_array(reference: &[u8]) -> Vec<bool> {
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

fn build_bucket_sizes(reference: &[u8]) -> Vec<usize> {
    let mut bucket_sizes = vec![0; ALPHABET.len()];

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

fn place_lms(suffix_array: &mut SuffixArray, reference: &[u8], lms_pointers: &[usize], mut bucket_tails: Vec<usize>) {
    for &i in lms_pointers {
        let c = reference[i] as usize;
        suffix_array[bucket_tails[c] - 1] = i;
        bucket_tails[c] -= 1;
    }
}

fn induce_l_types(suffix_array: &mut SuffixArray, reference: &[u8], t: &[bool], mut bucket_heads: Vec<usize>) {
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

fn induce_s_types(suffix_array: &mut SuffixArray, reference: &[u8], t: &[bool], mut bucket_tails: Vec<usize>) {
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

fn reduce_reference_string(reference: &[u8], suffix_array: &SuffixArray, lms_pointers: &[usize]) -> Vec<u8> {
    let mut reduced_string = Vec::new();
    let mut bucket_name = 0;
    let mut prev_lms_substring = suffix_array[0];

    reduced_string
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
    use crate::{read_genome, util::string_to_ints, HG38_1000_PATH};

    use super::*;
    use test::Bencher;

    #[test]
    fn test_type_map() {
        let reference = string_to_ints("ACATGA$");
        let t = build_type_array(&reference);
        assert_eq!(vec![true, false, true, false, false, false, true], t);
    }

    #[test]
    fn test_sais_mmiissiissiippii() {
        let reference = string_to_ints("CCAATTAATTAAGGAA$");
        let sa = suffix_array_induced_sort(&reference);
        assert_eq!(sa.len(), 99999999999999999);
    }

    #[test]
    fn test_sais_compare_naive() {
        let genome_string = "AATAAACCTTACCTAGCACTCCATCATGTCTTATGGCGCGTGATTTGCCCCGGACTCAGG$";
        let genome = string_to_ints(&genome_string);
        let naive = construct_suffix_array_naive(&genome);
        let sais = suffix_array_induced_sort(&genome);
        assert_eq!(naive, sais);
    }

    #[bench]
    #[ignore = "slow"]
    fn bench_sa_naive_ref1000(b: &mut Bencher) {
        let genome_string = read_genome(HG38_1000_PATH).unwrap();
        let genome = string_to_ints(&genome_string);
        b.iter(|| construct_suffix_array_naive(&genome))
    }

    #[bench]
    #[ignore = "slow"]
    fn bench_sais_ref1000(b: &mut Bencher) {
        let genome_string = read_genome(HG38_1000_PATH).unwrap();
        let genome = string_to_ints(&genome_string);
        b.iter(|| suffix_array_induced_sort(&genome))
    }
}
