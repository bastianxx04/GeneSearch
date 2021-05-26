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

    radix3(reference, alphabet_size, &mut sa12);

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
    radix_sort(reference, alphabet_size, &mut sa3, 0);
    merge(reference, sa12, sa3)
}

fn radix3(reference: &[u8], asize: usize, idx: &mut [usize]) {
    radix_sort(reference, asize, idx, 2);
    radix_sort(reference, asize, idx, 1);
    radix_sort(reference, asize, idx, 0);
}

fn radix_sort(reference: &[u8], asize: usize, idx: &mut [usize], offset: usize) {
    // Get the list of symbols to sort by
    let sort_symbols: Vec<usize> = idx
        .iter()
        .map(|i| safe_idx(reference, i + offset))
        .collect();

    // Get the bucket sizes
    let counts = count_symbols(sort_symbols, asize);
    let mut bucket_heads = cumulative_sums(counts);

    // Sort into a new array
    let mut sorted = vec![0; idx.len()];
    for i in idx.iter() {
        let bucket = safe_idx(reference, *i + offset);
        sorted[bucket_heads[bucket]] = *i;
        bucket_heads[bucket] += 1;
    }

    // Copy into input array
    idx.copy_from_slice(&sorted);
}

/// Return 0 if index is out of bounds
fn safe_idx(reference: &[u8], i: usize) -> usize {
    *(reference.get(i).unwrap_or(&0)) as usize
}

fn count_symbols(reference: Vec<usize>, asize: usize) -> Vec<usize> {
    let mut counts = vec![0; asize];
    for c in reference.iter() {
        counts[*c] += 1;
    }
    counts
}

fn cumulative_sums(counts: Vec<usize>) -> Vec<usize> {
    let mut res = vec![0; counts.len()];
    let mut acc = 0;
    for (i, k) in counts.iter().enumerate() {
        res[i] = acc;
        acc += k;
    }
    res
}

fn triplet(reference: &[u8], i: usize) -> Triplet {
    (
        safe_idx(reference, i),
        safe_idx(reference, i + 1),
        safe_idx(reference, i + 2),
    )
}

fn collect_alphabet(reference: &[u8], idx: &[usize]) -> TripletMap {
    let mut alpha: TripletMap = HashMap::new();
    for i in idx.iter() {
        let trip = triplet(reference, *i);
        let val = alpha.len() + 2;
        alpha.entry(trip).or_insert(val);
    }
    alpha
}

fn build_u(reference: &[u8], alpha: &TripletMap) -> Vec<u8> {
    let mut acc = Vec::new();

    for i in (1..reference.len()).step_by(3) {
        acc.push(alpha[&triplet(reference, i)] as u8)
    }
    acc.push(1);
    for i in (2..reference.len()).step_by(3) {
        acc.push(alpha[&triplet(reference, i)] as u8);
    }

    acc
}

fn u_idx(i: usize, m: usize) -> usize {
    if i < m {
        1 + 3 * i
    } else {
        2 + 3 * (i - m - 1)
    }
}

fn merge(reference: &[u8], sa12: Vec<usize>, sa3: Vec<usize>) -> Vec<usize> {
    let mut isa = HashMap::new();
    for (i, &v) in sa12.iter().enumerate() {
        isa.insert(v, i);
    }
    let mut sa = vec![];

    let mut i = 0;
    let mut j = 0;

    while i < sa12.len() && j < sa3.len() {
        if is_less(reference, sa12[i], sa3[j], &isa) {
            sa.push(sa12[i]);
            i += 1;
        } else {
            sa.push(sa3[j]);
            j += 1;
        }
    }
    sa.extend_from_slice(&sa12[i..]);
    sa.extend_from_slice(&sa3[j..]);
    sa
}

fn is_less(reference: &[u8], i: usize, j: usize, isa: &HashMap<usize, usize>) -> bool {
    let a = safe_idx(reference, i);
    let b = safe_idx(reference, j);
    if a < b {
        return true;
    }
    if a > b {
        return false;
    }
    if i % 3 != 0 && j % 3 != 0 {
        return isa[&i] < isa[&j];
    }
    is_less(reference, i + 1, j + 1, isa)
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
