use crate::suffix_array_construction::{find_bucket_heads, find_bucket_tails};
use crate::ALPHABET;
use num::{NumCast, Unsigned};

pub fn remap_string<T: Unsigned + NumCast>(s: &str) -> Vec<T> {
    s.chars()
        .flat_map(|c| ALPHABET.iter().position(|a| a == &c))
        .map(|c| match num::cast(c) {
            Some(v) => v,
            None => panic!("could not "),
        })
        .collect()
}

pub fn bwt(x: &[u8], sa: &[usize], i: usize) -> u8 {
    let x_index = sa[i];
    if x_index == 0 {
        x[x.len() - 1]
    } else {
        x[x_index - 1]
    }
}

// Gets the i'th row of BWM for x
#[allow(dead_code)]
pub fn bwm(x: &[u8], i: usize) -> String {
    let mut rows = vec![Vec::new(); x.len()];
    for j in 0..x.len() {
        let mut row = vec![0; x.len()];
        for k in 0..x.len() {
            row[k] = x[(j + k) % x.len()]
        }
        rows[j] = row;
    }
    rows.sort();

    let row = rows.remove(i);

    let mut res = String::new();
    for a in row {
        res.push(ALPHABET[a as usize])
    }
    res
}

pub fn print_sais_buckets(suffix_array: &[usize], bucket_sizes: &Vec<usize>, i: usize) {
    let bucket_heads = find_bucket_heads(&bucket_sizes);
    let bucket_tails = find_bucket_tails(&bucket_sizes);

    for i in 0..bucket_sizes.len() {
        let start = bucket_heads[i];
        let end = bucket_tails[i];
        let slice = &suffix_array[start..end];
        print!(
            "{:>3?}",
            slice
                .iter()
                .map(|&c| if c == usize::MAX { -1 } else { c as isize })
                .collect::<Vec<isize>>()
        );
    }

    println!();

    if i == usize::MAX {
        return;
    }

    // println!("{:?}", suffix_array);
    print!("{}", " ".repeat(i * 5 + 1));
    print!("  @");

    println!();
}
