use crate::ALPHABET;

pub fn string_to_ints(s: &str) -> Vec<u8> {
    s.chars()
        .map(|c| match c {
            '$' => 0,
            'a' => 1,
            'c' => 2,
            'g' => 3,
            't' => 4,
            _ => panic!("Bad string"),
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

pub fn bwt_string(x: &[u8], sa: &[usize]) -> String {
    let mut res = String::new();
    for i in 0..x.len() {
        res.push(ALPHABET[bwt(x, sa, i) as usize])
    }
    res
}
