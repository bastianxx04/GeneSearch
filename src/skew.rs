//! Implementation of the skew/DC3 algorithm.
//!
//! Implemented using https://github.com/mailund/pystr as a reference.

use std::collections::HashMap;

use crate::types::SuffixArray;

type Triplet = (usize, usize, usize);
type TripletMap = HashMap<Triplet, usize>;

pub fn skew(reference: &[u8]) -> SuffixArray {
    skew_rec(reference, 5)
}

fn skew_rec(reference: &[u8], alphabet_size: usize) -> SuffixArray {
    let mut sa12 = (0..reference.len())
        .filter(|&x| x % 3 != 0)
        .collect::<Vec<usize>>();
    sa12 = radix3(reference, alphabet_size, &sa12);
    let new_alpha = collect_alphabet(reference, &sa12);

    if new_alpha.len() < sa12.len() {
        let u = build_u(reference, &new_alpha);
        let sa_u = skew_rec(&u, new_alpha.len() + 2);
        let m = sa_u.len() / 2;
        sa12 = sa_u
            .iter()
            .filter(|&x| *x != m)
            .map(|&x| u_idx(x, m))
            .collect::<Vec<usize>>();
    }
    let mut sa3 = sa12
        .iter()
        .filter(|&x| x % 3 == 1)
        .map(|&x| x - 1)
        .collect::<Vec<usize>>();
    if reference.len() % 3 == 1 {
        sa3.insert(0, reference.len() - 1);
    }
    sa3 = bucket_sort(reference, alphabet_size, &sa3, 0);
    let result = merge(reference, sa12, sa3);

    result
}

fn safe_idx(reference: &[u8], i: usize) -> usize {
    if i >= reference.len() {
        0
    } else {
        reference[i] as usize
    }
}

fn symbcount(reference: Vec<usize>, asize: usize) -> Vec<usize> {
    let mut counts = vec![0; asize];
    for c in reference.iter() {
        counts[*c] += 1;
    }
    counts
}

fn cumsum(counts: Vec<usize>) -> Vec<usize> {
    let mut res = vec![0; counts.len()];
    let mut acc = 0;
    for (i, k) in counts.iter().enumerate() {
        res[i] = acc;
        acc += k;
    }
    res
}

fn bucket_sort<'a>(reference: &[u8], asize: usize, idx: &'a [usize], offset: usize) -> Vec<usize> {
    let mut sort_symbs = vec![];
    for i in idx {
        sort_symbs.push(safe_idx(reference, i + offset));
    }
    let counts = symbcount(sort_symbs, asize);
    let mut buckets = cumsum(counts);
    let mut out = vec![0; idx.len()];
    for i in idx.iter() {
        let bucket = safe_idx(reference, i + offset);
        out[buckets[bucket]] = *i;
        buckets[bucket] += 1;
    }
    return out.clone();
}

fn radix3<'a>(reference: &[u8], asize: usize, idx: &'a [usize]) -> Vec<usize> {
    let idx = bucket_sort(reference, asize, idx, 2);
    let idx = bucket_sort(reference, asize, &idx, 1);
    return bucket_sort(reference, asize, &idx, 0);
}

fn triplet(reference: &[u8], i: usize) -> Triplet {
    return (
        safe_idx(reference, i),
        safe_idx(reference, i + 1),
        safe_idx(reference, i + 2),
    );
}

fn collect_alphabet(reference: &[u8], idx: &[usize]) -> TripletMap {
    let mut alpha: TripletMap = HashMap::new();
    for i in idx.iter() {
        let trip = triplet(reference, *i);
        if !alpha.contains_key(&trip) {
            alpha.insert(trip, alpha.len() + 2);
        }
    }
    return alpha;
}

fn less(x: &[u8], i: usize, j: usize, isa: &HashMap<usize, usize>) -> bool {
    let a = safe_idx(x, i);
    let b = safe_idx(x, j);
    if a < b {
        return true;
    }
    if a > b {
        return false;
    }
    if i % 3 != 0 && j % 3 != 0 {
        return isa[&i] < isa[&j];
    }
    less(x, i + 1, j + 1, isa)
}

fn merge(reference: &[u8], sa12: Vec<usize>, sa3: Vec<usize>) -> Vec<usize> {
    let mut isa = HashMap::new();
    for i in 0..sa12.len() {
        isa.insert(sa12[i], i);
    }
    let mut sa = vec![];

    let mut i = 0;
    let mut j = 0;

    while i < sa12.len() && j < sa3.len() {
        if less(reference, sa12[i], sa3[j], &isa) {
            sa.push(sa12[i]);
            i += 1;
        } else {
            sa.push(sa3[j]);
            j += 1;
        }
    }
    sa.extend_from_slice(&sa12[i..]);
    sa.extend_from_slice(&sa3[j..]);
    return sa;
}

fn build_u(reference: &[u8], alpha: &TripletMap) -> Vec<u8> {
    let mut acc = Vec::new();
    let mut i = 1;
    while i < reference.len() {
        acc.push(alpha[&triplet(reference, i)] as u8);
        i += 3;
    }
    acc.push(1);
    i = 2;
    while i < reference.len() {
        acc.push(alpha[&triplet(reference, i)] as u8);
        i += 3;
    }
    return acc;
}

fn u_idx(i: usize, m: usize) -> usize {
    if i < m {
        return 1 + 3 * i;
    } else {
        return 2 + 3 * (i - m - 1);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::remap_reference;

    #[test]
    fn test_skew_mississippi() {
        let string = "CATTATTAGGA";
        let reference = remap_reference(&string);
        let result = skew(&reference);
        assert_eq!(result, vec![11, 10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2]);
    }
    #[test]
    fn test_skew_medium_string() {
        let reference =
            remap_reference("AATAAACCTTACCTAGCACTCCATCATGTCTTATGGCGCGTGATTTGCCCCGGACTCAGG");
        let result = skew(&reference);
        assert_eq!(
            result,
            vec![
                60, 3, 4, 0, 10, 5, 53, 17, 14, 57, 1, 22, 32, 25, 42, 16, 56, 21, 24, 20, 47, 48,
                49, 11, 6, 36, 50, 38, 12, 54, 18, 7, 29, 59, 52, 41, 15, 46, 35, 37, 58, 51, 34,
                27, 39, 2, 9, 13, 31, 55, 23, 19, 28, 40, 45, 33, 26, 8, 30, 44, 43
            ]
        );
    }
    #[test]
    fn test_skew_aaaaaa() {
        let reference = remap_reference("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
        let result = skew(&reference);
        for i in 0..reference.len() {
            assert_eq!(result[i], reference.len() - i - 1);
        }
    }
}
