use std::collections::BTreeSet;
use crate::ALPHABET;
use crate::OTable;
use crate::CTable;
use crate::DTable;

/// Approximative search
pub fn approx_search(input_word: &[u8], search_word: &[u8], o_table: &OTable, c_table: &CTable, diffs: usize) -> Vec<(usize, usize)> {
    let d_table = calculate_d_table(input_word, search_word, c_table, o_table);
    let ret = inexact_recursion(search_word, search_word.len()-1, diffs, 1, input_word.len()-1, &d_table, &o_table, &c_table);

    println!("we got {:?}", ret);

    return Vec::new()
}

fn calculate_d_table(input_word: &[u8], search_word: &[u8], c_table: &CTable, o_table: &OTable) -> DTable {
    let mut start = 1;
    println!("input word: {:?}", input_word);
    println!("search word: {:?}", search_word);
    let mut end = input_word.len() - 1;
    let mut diffs = 0;
    let mut D: DTable = Vec::new();

    for i in 0..(search_word.len()) {
        let i_char_num = usize::from(search_word[i]);
        println!("i_char: {}", i_char_num);
        println!("start: {} - end: {} - i: {}", start, end, i);
        println!("c: {} o: {}", c_table[i_char_num], o_table[end][i_char_num]);
        start = c_table[i_char_num] + o_table[start - 1][i_char_num] + 1;
        end = c_table[i_char_num] + o_table[end][i_char_num]-1;
        if start > end {
            println!("  start was bigger than end");
            start = 1;
            end = input_word.len()-1;
            diffs += 1;
        }
        D.push(diffs);
    }
    println!("made D table with values: {:?}", D);
    return D
}

fn inexact_recursion(search_word: &[u8], level: usize, diffs: usize, mut start_pos: usize, mut end_pos: usize, d_table: &DTable, o_table: &OTable, c_table: &CTable) -> BTreeSet<(usize, usize)>{
    println!("entered recursive loop at level: {}", level); 
    if diffs < d_table[level] { 
        println!("  returned nothing");
        return BTreeSet::new()
    }
    if level < 0 {
        let set: BTreeSet<(usize, usize)> = [(start_pos, end_pos)].iter().cloned().collect();
        println!("  returned something: {:?}", set);
        return set
    }
    let mut I = BTreeSet::new();
    I.union(&inexact_recursion(search_word, level-1, diffs-1, start_pos, end_pos, d_table, o_table, c_table));
    for (b, _) in ALPHABET.iter().enumerate() { 
        println!("  ran loop in level: {}, checking b: {}", level, b);
        start_pos = c_table[b] + o_table[start_pos-1][b] + 1;
        end_pos = c_table[b] + o_table[end_pos][b];
        println!("  start: {} - end: {}", start_pos, end_pos);
            if start_pos <= end_pos {
                println!("  start pos was smaller than end pos");
                I = I.union(&inexact_recursion(search_word, level, diffs-1, start_pos, end_pos, d_table, o_table, c_table)).cloned().collect();
                if b == search_word[level].into() {
                    println!("      case exact match!");
                    I = I.union(&inexact_recursion(search_word, level-1, diffs, start_pos, end_pos, d_table, o_table, c_table)).cloned().collect();
                } else {
                    println!("      case found a difference");
                    I = I.union(&inexact_recursion(search_word, level-1, diffs-1, start_pos, end_pos, d_table, o_table, c_table)).cloned().collect();
                }
            }
        }
    return I
}