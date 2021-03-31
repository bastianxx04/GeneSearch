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
    let t = build_type_array(reference);        //build type map VERIFIED
    println!("typemap: {:?}", t);
    let lms_pointers = build_lms_array(&t);    //get array of start positions of lms strings  

    let bucket_sizes = build_bucket_sizes(reference, alphabet_size);
    let bucket_heads = find_bucket_heads(&bucket_sizes);        //make all the bucket info
    let bucket_tails = find_bucket_tails(&bucket_sizes);
    println!("bucket sizes: {:?}", bucket_sizes);
    println!("bucket tails: {:?}", bucket_tails);
    
    let mut suffix_array = vec![usize::MAX; n+1];
    println!("reference {:?} has len {}, and sa has len {}", reference, reference.len(), suffix_array.len());
    place_lms(&mut suffix_array, reference, &t, bucket_tails.clone()); //Place all suffixes into approxemately the right position
    
    println!("Induce sort L");
    induce_l_types(&mut suffix_array, reference, &t, bucket_heads.clone()); //Slot all the other suffixes into the guessed suffix array
    println!("Induce sort S");
    induce_s_types(&mut suffix_array, reference, &t, bucket_tails.clone());
    
    println!("summarise suffix array:");
    let (reduced_string, new_alphabet_len, offsets) = reduce_reference_string(reference, &suffix_array, &lms_pointers, &bucket_tails, &t);
    
    println!("make summary suffix array:");
    let summary_suffix_array = make_summary_suffix_array(&reduced_string, new_alphabet_len); //gives wrong result, proceeding anyways
    
    println!("accurate lms sort:");
    let mut result = accurate_lms_sort(&reference, bucket_tails.clone(), &t, &summary_suffix_array, &offsets);
    
    println!("Induce sort L");
    induce_l_types(&mut result, reference, &t, bucket_heads.clone()); //Slot all the other suffixes into the guessed suffix array
    println!("Induce sort S");
    induce_s_types(&mut result, reference, &t, bucket_tails.clone());

    println!("FINAL SCORE: {:?}", result);
    result
}

fn accurate_lms_sort(reference: &Vec<usize>, mut bucket_tails: Vec<usize>, t: &Vec<bool>, summary_suffix_array: &Vec<usize>, offsets: &Vec<usize>) -> Vec<usize> {
    let mut suffix_offsets = vec![usize::MAX; reference.len() + 1];

    for i in (1..summary_suffix_array.len()).rev() { //maybe a off by one here? find out later.
        let string_index = offsets[summary_suffix_array[i]];

        let bucket_index = if string_index == reference.len() {
            continue;
        }else {
            reference[string_index]
        };

        suffix_offsets[bucket_tails[bucket_index]] = string_index;

        bucket_tails[bucket_index] -= 1;

        println!("{:?}", suffix_offsets);
    }
    
    suffix_offsets[0] = reference.len();
    println!("{:?}", suffix_offsets);

    suffix_offsets
}

fn make_summary_suffix_array(reduced_string: &Vec<usize>, alphabet_size: usize) -> Vec<usize> {
    let mut summary_suffix_array = Vec::new();

    if alphabet_size == reduced_string.len(){
        summary_suffix_array = vec![usize::MAX; reduced_string.len() + 1];

        summary_suffix_array[0] = reduced_string.len();

        for i in 0..reduced_string.len(){
            let y = reduced_string[i];
            summary_suffix_array[y+1] = i
        }

    } else {
        println!("ENTERED RECURSION WITH REDUCED STRING {:?} ALPHABET SIZE {}", reduced_string, alphabet_size);
        summary_suffix_array = suffix_array_induced_sort(reduced_string, alphabet_size);
    }
    println!("{:?}", summary_suffix_array);
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

fn is_lms_char(offset: usize, typemap: &[bool]) -> bool {
    if offset == 0 {
        return false
    }
    (typemap[offset] == true && typemap[offset-1] == false)
}

fn place_lms(suffix_array: &mut SuffixArray, reference: &[usize], t: &[bool], mut bucket_tails: Vec<usize>) {

    println!("GuessLMSSort:");
    for i in 0..reference.len() {
        if !is_lms_char(i, t){
            continue;
        }

        let bucket_index = reference[i];
        suffix_array[bucket_tails[bucket_index]] = i;
        bucket_tails[bucket_index] -= 1;

        println!("{:?}", suffix_array);
    }
    
    suffix_array[0] = reference.len();
    println!("{:?}", suffix_array);
}

fn induce_l_types(suffix_array: &mut SuffixArray, reference: &[usize], t: &[bool], mut bucket_heads: Vec<usize>) {
    // STEP 2 (it's about to get crazy)
    let n = reference.len();
    for i in 0..suffix_array.len() {
        if suffix_array[i] == usize::MAX {
            println!("continued cuz i:{} is max", i);
            continue;
        }
        
        if (suffix_array[i]) == 0 {
            println!("continued cuz suff array at i:{} is 0", i);
            continue;
        }
        let j = suffix_array[i] - 1;
        
        if t[j] {continue;}
        
        let bucket_index = reference[j];
        println!("bucket index: {} - bucket heads: {:?}", bucket_index, bucket_heads);
        suffix_array[bucket_heads[bucket_index]] = j;
        bucket_heads[bucket_index] += 1;
        
        println!("{:?} - looking at _", suffix_array, );
    }
}

fn induce_s_types(suffix_array: &mut SuffixArray, reference: &[usize], t: &[bool], mut bucket_tails: Vec<usize>) {
    // STEP 3 (the one where the magic happens)
    for i in (0..suffix_array.len()-1).rev() {
        
        if (suffix_array[i] as isize - 1) < 0 {continue;}
        let j = suffix_array[i] - 1;

        if !t[j] {continue;} //only interested in s types

        let bucket_index = reference[j];
        suffix_array[bucket_tails[bucket_index]] = j;
        bucket_tails[bucket_index] -= 1;

        println!("{:?} - looking at bucket index {}, ref: {:?}, j: {}", suffix_array, bucket_index, reference, j);
    }
}

fn reduce_reference_string(reference: &[usize], suffix_array: &SuffixArray, lms_pointers: &[usize], bucket_tails: &Vec<usize>, t: &[bool]) -> (Vec<usize>, usize, Vec<usize>) {
    let mut lms_names = vec![usize::MAX; reference.len() + 1];
    let mut current_name = 0;
    let mut last_lms_suffix_offset = usize::MAX;

    lms_names[suffix_array[0]] = current_name;
    let mut prev_lms_substring = suffix_array[0];

    println!("lms names:");
    for i in 1..(suffix_array.len()-2) {
        let suffix_offset = suffix_array[i];

        if !is_lms_char(suffix_offset, &t) { //We only case about lms suffixes
            continue;
        }

        if !compare_lms(reference, t, prev_lms_substring, suffix_offset) {    //if this lms suffix starts with a different lms substring from the last one, we give it a new current_name
            current_name += 1;
        }

        prev_lms_substring = suffix_offset;
        lms_names[suffix_offset] = current_name;

        println!("{:?}", lms_names)
    }


    let mut offsets = Vec::new();
    let mut reduced_string = Vec::new();
    for i in 0..lms_names.len() {
        let name = lms_names[i];
        if name == usize::MAX {
            continue;
        }
        offsets.push(i);
        reduced_string.push(name);
    }

    let new_alphabet_size = current_name+1;

    println!("reduced ref string: {:?}", reduced_string);
    (reduced_string, new_alphabet_size, offsets)
}

fn compare_lms(reference: &[usize], t: &[bool], i: usize, j: usize) -> bool {
    if i == j {
        return true
    }

    let n = reference.len();
    if i == n || j == n {
        return false
    }
    
    let mut k = 0;
    loop {
        let i_is_lms = is_lms_char(k+i, &t);
        let j_is_lms = is_lms_char(k+j, &t);
        if k > 0 && i_is_lms && j_is_lms {
            return true
        }

        if i_is_lms != j_is_lms {
            return false
        }

        if reference[i + k] != reference[j + k] {
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

    #[test]
    fn test_sais_compare_cabbage() {
        let genome = vec![2, 0, 1, 1, 0, 6, 4];
        //let naive = construct_suffix_array_naive(&genome);
        let sais = suffix_array_induced_sort(&genome.iter().map(|&a| a as usize).collect(), 7);
        assert!(false);
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
