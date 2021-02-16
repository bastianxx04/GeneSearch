fn main() {
    //OUR CURRENT ALPHABET IS A, C, G, T
    let input_string = "ataata";
    println!("  === INPUT IS \"{}\" ===", input_string);

    let mut input_array: Vec<_> = input_string.chars().collect();
    input_array.push('$');

    //Make the Data Array   [Suffix], num, [Rank Table]
    let data_table: Vec<(Vec<char>, usize, Vec<usize>)> = suff_gen(input_array);

    let mut all_suffixes_ordered: Vec<(Vec<char>, usize, Vec<usize>)> = data_table.clone();
    all_suffixes_ordered.sort();
    
    print!("\n\nGENERATED BWT MATRIX - SORTED");
    for elem in &all_suffixes_ordered {
        print!("\n    ");
        for inner_elem in &elem.0 {
            print!("{} ", inner_elem);
        }
        print!("    {}", elem.1);
    }

    let mut f_and_l_ordered: Vec<(Vec<char>, usize, Vec<usize>)> = rank_table_gen(all_suffixes_ordered.clone());
    for i in 0..(data_table.len()) {
        let suff = all_suffixes_ordered[i].0.clone();
        f_and_l_ordered[i].0 = vec![suff[0], *suff.last().unwrap()];
    }

    print!("\n\nF & L - SORTED");
    print!("\n  F  L    num     A C G T");
    for elem in f_and_l_ordered {
        print!("\n  ");
        for inner_elem in elem.0 {
            print!("{}  ", inner_elem);
        }
        print!("   {}", elem.1);
        print!("      {} {} {} {}", elem.2[0],elem.2[1],elem.2[2],elem.2[3]);
    }
}

fn suff_gen(input_array: Vec<char>) -> Vec<(Vec<char>, usize, Vec<usize>)>{
    let mut temp_data_table: Vec<(Vec<char>, usize, Vec<usize>)> = Vec::new();
    for i in 0..(input_array.len()) {
        let mut to_be_inserted = input_array.clone();
        let rank_table = Vec::new();

        if temp_data_table.len() > 0{
            to_be_inserted = temp_data_table.last().unwrap().0.to_vec();
            let last_elem = to_be_inserted.pop().unwrap();
            to_be_inserted.insert(0, last_elem);
        }
        temp_data_table.push((to_be_inserted, i, rank_table));
    }
    return temp_data_table;
}

fn rank_table_gen(mut bwt: Vec<(Vec<char>, usize, Vec<usize>)>) -> Vec<(Vec<char>, usize, Vec<usize>)>{
    //ORDER: A, C, G, T
    let mut old_rank_table: Vec<usize> = vec![0,0,0,0];
    for i in 0..(bwt.len()) {
        let mut new_rank_table: Vec<usize> = old_rank_table.clone();
        if *bwt[i].0.last().unwrap() == 'a' {
            new_rank_table[0] += 1;
        }
        if *bwt[i].0.last().unwrap() == 'c' {
            new_rank_table[1] += 1;
        }
        if *bwt[i].0.last().unwrap() == 'g' {
            new_rank_table[2] += 1;
        }
        if *bwt[i].0.last().unwrap() == 't' {
            new_rank_table[3] += 1;
        }
        bwt[i].2 = new_rank_table.clone();
        old_rank_table = new_rank_table.clone();
    }
    return bwt;
}

//TODO: GENERATE RANK TABLES
//TODO: MAKE 'QUERY' FUNCTION (FROM X IN F TO Y IN L)
//TODO: MAKE 'MATCH' FUNCTION THAT CALLS QUERY RECURSIVELY
//TODO: GENERATE F & L MORE EFFICIENTLY
//TODO: MAKE SKIPS IN RANKT TABLES
//TODO: MAKE SKIPS IN NUMBERING TABLE
//TODO: WRITE ABOUT IT
