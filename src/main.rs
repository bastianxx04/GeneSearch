mod suffix_array_construction;
mod exact_search;
//mod approx_search;

use suffix_array_construction::construct_suffix_array_naive;
use exact_search::exact_search;

fn main() {
    //OUR CURRENT ALPHABET IS A, C, G, T
    let input_string = "ataata";
    println!("  === INPUT IS \"{}\" ===", input_string);

    let mut input_array: Vec<_> = input_string.chars().collect();
    input_array.push('$');

    //Make the Data Array   [Suffix], num, [Rank Table]
    let suffix_array = construct_suffix_array_naive(&input_array);
    
    //pretty_print_bwt(&all_suffixes_ordered);

    let bwt_ordered = rank_table_gen(&suffix_array, &input_array);

    pretty_print_bwt_rank(&bwt_ordered, &input_array);

    let seach_result = exact_search(&suffix_array, &input_array, &bwt_ordered);
}

fn pretty_print_bwt_rank(bwt_ordered: &[(usize, Vec<usize>)], input_array: &[char]) {
    print!("\n\nF & L - SORTED");
    print!("\n  F  L    num     A C G T");
    for (num_ref, rank) in bwt_ordered {
        let num = *num_ref;
        print!("\n  ");
        if num == 0 {
            print!("{}  $  ", input_array[0]);
        } else {
            print!("{}  {}  ", input_array[num], input_array[num - 1]);
        }
        print!("   {}", num);
        print!("      {} {} {} {}", rank[0], rank[1], rank[2], rank[3]);
    }
}

fn rank_table_gen(suffix_array: &[usize], input_array: &[char]) -> Vec<(usize, Vec<usize>)>{
    //ORDER: A, C, G, T
    let mut old_rank_table = vec![0,0,0,0];
    let mut new_bwt_output = Vec::new();
    for i in 0..suffix_array.len() {
        let mut new_rank_table = old_rank_table.clone();
        match input_array[suffix_array[i]] {
            'a' => new_rank_table[0] += 1,
            'c' => new_rank_table[1] += 1,
            'g' => new_rank_table[2] += 1,
            't' => new_rank_table[3] += 1,
            _ => {}
        }

        old_rank_table = new_rank_table.clone();
        new_bwt_output.push((suffix_array[i], new_rank_table.clone()))
    }
    new_bwt_output
}

//TODO: MAKE 'QUERY' FUNCTION (FROM X IN F TO Y IN L)
//TODO: MAKE 'MATCH' FUNCTION THAT CALLS QUERY RECURSIVELY
//TODO: GENERATE F & L MORE EFFICIENTLY
//TODO: MAKE SKIPS IN RANKT TABLES
//TODO: MAKE SKIPS IN NUMBERING TABLE
//TODO: WRITE ABOUT IT
