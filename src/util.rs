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

// Gets the i'th row of BWM for x
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
    return res;
}
