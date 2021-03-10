use std::collections::BTreeSet;
use crate::ALPHABET;
use crate::OTable;
use crate::CTable;
use crate::DTable;
use std::convert::TryFrom;

/// Approximative search
pub fn approx_search(input_word: &[u8], search_word: &[u8], o_table: &OTable, c_table: &CTable, edits_left: usize) -> Vec<(usize, usize)> {
    let d_table = calculate_d_table(input_word, search_word, c_table, o_table);
    let ret = inexact_recursion(search_word, (search_word.len()-1) as i32, edits_left as i32, 1, input_word.len()-1, &d_table, &o_table, &c_table);

    println!("we got {:?}", ret);

    return Vec::new()
}

fn calculate_d_table(input_word: &[u8], search_word: &[u8], c_table: &CTable, o_table: &OTable) -> DTable {
    let mut start = 1;
    println!("input word: {:?}", input_word);
    println!("search word: {:?}", search_word);
    let mut end = input_word.len() - 1;
    let mut edits_left = 0;
    let mut D: DTable = Vec::new();

    for i in 0..(search_word.len()) {
        let i_char_num = usize::from(search_word[i]);
        println!("i_char: {}", i_char_num);
        println!("start: {} - end: {} - i: {}", start, end, i);
        println!("c: {} o: {}", c_table[i_char_num], o_table[end][i_char_num]);
        start = c_table[i_char_num] + o_table[start - 1][i_char_num];
        end = c_table[i_char_num] + o_table[end][i_char_num]-1;
        if start > end {
            println!("  start was bigger than end");
            start = 1;
            end = input_word.len()-1;
            edits_left += 1;
        }
        D.push(edits_left);
    }
    println!("made D table with values: {:?}", D);
    return D
}

fn inexact_recursion(search_word: &[u8], i: i32, edits_left: i32, left: usize, right: usize, d_table: &DTable, o_table: &OTable, c_table: &CTable) -> BTreeSet<(usize, usize)>{
    println!("entered recursive loop at level: {}", i); 
    
    let lower_limit = match usize::try_from(i) {
        Ok(value) => d_table[value], 
        Err(_) => 0,
    };
    
    if edits_left < lower_limit as i32 {
        println!("  returned nothing");
        return BTreeSet::new()
    }

    if i < 0 {
        let set: BTreeSet<(usize, usize)> = [(left, right)].iter().cloned().collect();
        println!("  returned something: {:?}", set);
        return set
    }

    let mut I = BTreeSet::new();
    I.union(&inexact_recursion(search_word, i-1, edits_left-1, left, right, d_table, o_table, c_table));
    
    for (b, _) in ALPHABET.iter().enumerate() { 
        println!("pre-math  start: {} - end: {}", left, right);
        let left = c_table[b] + o_table[left][b];
        let right = c_table[b] + o_table[right][b];
        println!("post-math  start: {} - end: {}", left, right);
        if left <= right {
            I = I.union(&inexact_recursion(search_word, i, edits_left-1, left, right, d_table, o_table, c_table)).cloned().collect();
            if b == search_word[i as usize].into() {
                I = I.union(&inexact_recursion(search_word, i-1, edits_left, left, right, d_table, o_table, c_table)).cloned().collect();
            } else {
                I = I.union(&inexact_recursion(search_word, i-1, edits_left-1, left, right, d_table, o_table, c_table)).cloned().collect();
            }
        }
    }
    return I
} 