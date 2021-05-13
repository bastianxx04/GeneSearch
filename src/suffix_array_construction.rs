use crate::types::SuffixArray;

/// Construct a suffix array naively
pub fn construct_suffix_array_naive(reference: &[u8]) -> SuffixArray {
    let mut suffixes: Vec<(&[u8], usize)> = Vec::with_capacity(reference.len());

    for i in 0..(reference.len()) {
        let suffix = &reference[i..];
        suffixes.push((suffix, i));
    }

    suffixes.sort();

    suffixes.iter().map(|elem| elem.1).collect()
}
