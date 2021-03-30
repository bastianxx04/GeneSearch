use std::usize;
use crate::types::SuffixArray;
use crate::ALPHABET;


/// SA-IS
#[allow(dead_code)]
pub fn suffix_array_induced_sort(reference: &Vec<usize>, alphabet_size: usize) -> SuffixArray {
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
    let t = build_type_array(reference);        //build type map
    let lms_pointers = build_lms_array(&t);    //get array of start positions of lms strings 

    let bucket_sizes = build_bucket_sizes(reference, alphabet_size);
    let bucket_heads = find_bucket_heads(&bucket_sizes);        //make all the bucket info
    let bucket_tails = find_bucket_tails(&bucket_sizes);
    
    let mut suffix_array = vec![usize::MAX; n];
    place_lms(&mut suffix_array, reference, &lms_pointers, bucket_tails.clone()); //Place all suffixes into approxemately the right position

    induce_l_types(&mut suffix_array, reference, &t, bucket_heads.clone()); //Slot all the other suffixes into the guessed suffix array
    induce_s_types(&mut suffix_array, reference, &t, bucket_tails.clone());
    
    println!("suffix array: {:?}", suffix_array);
    let (reduced_string, new_alphabet_len, offsets) = reduce_reference_string(reference, &suffix_array, &lms_pointers, &bucket_tails, &t);
    
    let summary_suffix_array = make_summary_suffix_array(&reduced_string, new_alphabet_len);
    
    let mut result = accurate_lms_sort(&reference, bucket_tails.clone(), &t, &summary_suffix_array, &offsets);

    induce_l_types(&mut result, reference, &t, bucket_heads.clone()); //Slot all the other suffixes into the guessed suffix array
    induce_s_types(&mut result, reference, &t, bucket_tails.clone());

    /*
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
    println!("sa1: {:?}",sa1);
    */

    result
}

fn accurate_lms_sort(reference: &Vec<usize>, mut bucket_tails: Vec<usize>, t: &Vec<bool>, summary_suffix_array: &Vec<usize>, offsets: &Vec<usize>) -> Vec<usize> {
    let mut suffix_offsets = vec![usize::MAX; reference.len() + 1];

    for i in (0..summary_suffix_array.len()).rev() { //maybe a off by one here? find out later.
        let string_index = offsets[summary_suffix_array[i]];

        let bucket_index = reference[string_index];

        suffix_offsets[bucket_tails[bucket_index]] = string_index;

        bucket_tails[bucket_index] -= 1;
    }

    suffix_offsets[0] = reference.len();

    suffix_offsets
}

fn make_summary_suffix_array(reduced_string: &Vec<usize>, alphabet_size: usize) -> Vec<usize> {
    let mut summary_suffix_array = Vec::new();

    if alphabet_size == reduced_string.len(){
        summary_suffix_array = vec![usize::MAX; reduced_string.len() + 1];

        summary_suffix_array[0] = reduced_string.len();

        for i in 0..reduced_string.len(){
            summary_suffix_array[reduced_string[i]+1] = i
        }

    } else {
        println!("ENTERED RECURSION WITH REDUCED STRING {:?} ALPHABET SIZE {}", reduced_string, alphabet_size);
        summary_suffix_array = suffix_array_induced_sort(reduced_string, alphabet_size);
    }
    summary_suffix_array
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

fn place_lms(suffix_array: &mut SuffixArray, reference: &[usize], lms_pointers: &[usize], mut bucket_tails: Vec<usize>) {
    for &i in lms_pointers {
        let c = reference[i] as usize;
        suffix_array[bucket_tails[c] - 1] = i;
        bucket_tails[c] -= 1;
    }
}

fn induce_l_types(suffix_array: &mut SuffixArray, reference: &[usize], t: &[bool], mut bucket_heads: Vec<usize>) {
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

fn induce_s_types(suffix_array: &mut SuffixArray, reference: &[usize], t: &[bool], mut bucket_tails: Vec<usize>) {
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

fn reduce_reference_string(reference: &[usize], suffix_array: &SuffixArray, lms_pointers: &[usize], bucket_tails: &Vec<usize>, t: &[bool]) -> (Vec<usize>, usize, Vec<usize>) {
    let n = reference.len();
    let mut lms_names = vec![usize::MAX; n + 1];
    let mut current_name = 0;
    let mut last_lms_suffix_offset = usize::MAX;

    lms_names[suffix_array[0]] = current_name;
    let mut prev_lms_substring = suffix_array[0];

    for i in 1..(n) {
        let suffix_offset = suffix_array[i];

        if !lms_pointers.contains(&suffix_offset) { //We only case about lms suffixes
            continue;
        }

        if !compare_lms(reference, t, lms_pointers, prev_lms_substring, suffix_offset) {    //if this lms suffix starts with a different lms substring from the last one, we give it a new current_name
            current_name += 1;
        }

        prev_lms_substring = suffix_offset;
        lms_names[suffix_offset] = current_name;
    }


    let mut offsets = Vec::new();
    let mut reduced_string = Vec::new();
    for i in 0..lms_names.len() {
        let name = lms_names[i];
        if name == usize::MAX {
            continue;
        }
        offsets.push(i);
        reduced_string.push(current_name);
    }

    let new_alphabet_size = current_name+1;

    println!("reduced ref string: {:?}", reduced_string);
    (reduced_string, new_alphabet_size, offsets)
}

fn compare_lms(reference: &[usize], t: &[bool], lms_pointers: &[usize], i: usize, j: usize) -> bool {
    if i == j {
        return true
    }

    let n = reference.len();
    if i == n - 1 || j == n - 1 {
        return false
    }
    
    let mut k = 0;
    loop {
        if k > 0 && lms_pointers.contains(&(i + k)) && lms_pointers.contains(&(j + k)) {
            return true
        }

        if reference[i + k] != reference[j + k] || t[i + k] != t[j + k] {
            return false
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
    use crate::{read_genome, util::string_to_ints, HG38_1000_PATH};

    use super::*;
    use test::Bencher;

    #[test]
    fn test_type_map() {
        let reference = string_to_ints("ACATGA$");
        let t = build_type_array(&reference.iter().map(|&a| a as usize).collect());
        assert_eq!(vec![true, false, true, false, false, false, true], t);
    }

    #[test]
    fn test_sais_mmiissiissiippii() {
        let reference = string_to_ints("CCAATTAATTAAGGAA$");
        let sa = suffix_array_induced_sort(&reference.iter().map(|&a| a as usize).collect(), ALPHABET.len());
        assert_eq!(vec![16, 15, 14, 10, 6, 2, 11, 7, 3, 1, 0, 13, 12, 9, 5, 8, 4], sa);
    }

    #[test]
    fn test_sais_compare_naive() {
        let genome_string = "AATAAACCTTACCTAGCACTCCATCATGTCTTATGGCGCGTGATTTGCCCCGGACTCAGG$";
        let genome = string_to_ints(&genome_string);
        let naive = construct_suffix_array_naive(&genome);
        let sais = suffix_array_induced_sort(&genome.iter().map(|&a| a as usize).collect(), ALPHABET.len());
        for i in 0..sais.len() {
            if sais[i] != naive[i] {
                let print = &genome[sais[i]..];
                println!("{}: {:?}, ", i, print);
            }
        }
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
        b.iter(|| suffix_array_induced_sort(&genome.iter().map(|&a| a as usize).collect(), ALPHABET.len()))
    }
}
