use crate::types::SuffixArray;
use crate::util::print_sais_buckets;

const UNDEFINED: usize = usize::MAX;

/// SA-IS
#[allow(dead_code)]
pub fn suffix_array_induced_sort(reference: &Vec<u32>, alphabet_size: usize) -> SuffixArray {
    println!("Entered the recursive function.");
    println!(
        "{:>3?} INDICES",
        (0..reference.len()).collect::<Vec<usize>>()
    );
    println!("{:>3?} REFERENCE", reference);

    let t = build_type_array(reference);
    println!(
        "{:?} TYPEMAP",
        t.iter()
            .map(|&b| if b { 'S' } else { 'L' })
            .collect::<Vec<char>>()
    );

    let p1 = build_lms_array(&t);
    println!("{:?} LMS POINTERS", p1);

    println!();

    let bucket_sizes = build_bucket_sizes(reference, alphabet_size);

    let mut suffix_array = vec![UNDEFINED; reference.len()];
    place_lms(reference, &mut suffix_array, &p1, &bucket_sizes);
    induce_l_types(reference, &mut suffix_array, &t, &bucket_sizes);
    induce_s_types(reference, &mut suffix_array, &t, &bucket_sizes);

    let (reduced_string, reduced_offsets, new_alphabet_size) =
        reduce_reference_string(reference, &suffix_array, &p1, &t);

    let reduced_sa = compute_reduced_suffix_array(&reduced_string, new_alphabet_size);

    // Induce SA from SA1
    remap_lms(
        reference,
        &mut suffix_array,
        &reduced_string,
        &reduced_offsets,
        &reduced_sa,
        &bucket_sizes,
    );
    induce_l_types(reference, &mut suffix_array, &t, &bucket_sizes);
    induce_s_types(reference, &mut suffix_array, &t, &bucket_sizes);

    println!("Exiting the recursive function.");
    suffix_array
}

fn build_type_array(reference: &Vec<u32>) -> Vec<bool> {
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
    lms_pointers: &[usize],
    bucket_sizes: &Vec<usize>,
) {
    let mut bucket_tails = find_bucket_tails(bucket_sizes);

    for &i in lms_pointers {
        let c = reference[i];
        bucket_tails[c as usize] -= 1;
        suffix_array[bucket_tails[c as usize]] = i;
    }
}

fn induce_l_types(
    reference: &[u32],
    suffix_array: &mut SuffixArray,
    t: &[bool],
    bucket_sizes: &Vec<usize>,
) {
    let mut bucket_heads = find_bucket_heads(bucket_sizes);

    for i in 0..reference.len() {
        print_sais_buckets(&suffix_array, bucket_sizes, i);

        if suffix_array[i] == usize::MAX || suffix_array[i] == 0 {
            continue;
        }

        let j = suffix_array[i] - 1;

        if !t[j] {
            let c = reference[j] as usize;
            suffix_array[bucket_heads[c]] = j;
            bucket_heads[c] += 1;
        }
    }
}

fn induce_s_types(
    reference: &[u32],
    suffix_array: &mut SuffixArray,
    t: &[bool],
    bucket_sizes: &Vec<usize>,
) {
    let mut bucket_tails = find_bucket_tails(bucket_sizes);

    for i in (0..reference.len()).rev() {
        if suffix_array[i] == usize::MAX || suffix_array[i] == 0 {
            continue;
        }

        let j = suffix_array[i] - 1;
        if t[j] {
            let c = reference[j] as usize;
            bucket_tails[c] -= 1;
            suffix_array[bucket_tails[c]] = j;
        }
    }
}

fn reduce_reference_string(
    reference: &[u32],
    suffix_array: &SuffixArray,
    lms_pointers: &[usize],
    t: &[bool],
) -> (Vec<u32>, Vec<usize>, usize) {
    let n = reference.len();
    let mut names_buf = vec![usize::MAX; n + 1];
    let mut name = 0;

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

    let mut reduced_string = Vec::new();
    let mut reduced_offsets = Vec::new();
    for i in 0..(n + 1) {
        name = names_buf[i];
        if name == UNDEFINED {
            continue;
        }
        reduced_offsets.push(i);
        reduced_string.push(name as u32);
    }

    println!("reduced ref string: {:?}", reduced_string);
    (reduced_string, reduced_offsets, new_alphabet_size)
}

fn compare_lms(reference: &[u32], t: &[bool], lms_pointers: &[usize], i: usize, j: usize) -> bool {
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

/// Computes the suffix array for the reduced string.
/// If each character in the reduced string is unique, it is computed directly.
/// Otherwise, it is computed using the SA-IS algorithm recursively.
fn compute_reduced_suffix_array(reduced_string: &Vec<u32>, alphabet_size: usize) -> Vec<usize> {
    let n = reduced_string.len();
    if n == alphabet_size {
        // There are no duplicates in the reduced string
        let mut reduced_sa = vec![0; n];
        reduced_sa[0] = alphabet_size;
        for i in 0..n {
            let j = reduced_string[i] as usize;
            reduced_sa[j] = i;
        }

        reduced_sa
    } else {
        // There are duplicates in the reduced string
        suffix_array_induced_sort(reduced_string, alphabet_size)
    }
}

fn remap_lms(
    reference: &[u32],
    suffix_array: &mut [usize],
    reduced_string: &[u32],
    reduced_offsets: &[usize],
    reduced_sa: &[usize],
    bucket_sizes: &Vec<usize>,
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
    fn test_sais_compare_naive_mmiissiissiippii() {
        let genome_u8 = remap_string("CCAATTAATTAAGGAA$");
        let naive = construct_suffix_array_naive(&genome_u8);
        let genome_u32 = genome_u8.iter().map(|&c| c as u32).collect();
        let sais = suffix_array_induced_sort(&genome_u32, ALPHABET.len());
        for i in 0..sais.len() {
            if sais[i] != naive[i] {
                let print = &genome_u32[sais[i]..];
                println!("{}: {:?}, ", i, print);
            }
        }
        assert_eq!(naive, sais);
    }

    #[test]
    fn test_sais_compare_naive() {
        let genome_u8 = remap_string(GEN60);
        let naive = construct_suffix_array_naive(&genome_u8);
        let genome_u32 = genome_u8.iter().map(|&c| c as u32).collect();
        let sais = suffix_array_induced_sort(&genome_u32, ALPHABET.len());
        for i in 0..sais.len() {
            if sais[i] != naive[i] {
                let print = &genome_u32[sais[i]..];
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
