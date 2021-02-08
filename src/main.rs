fn main() {
    //OUR CURRENT ALPHABET IS A, C, G, T
    let input_string = "ataata";
    println!("  === INPUT IS \"{}\" ===", input_string);

    let mut input_array: Vec<_> = input_string.chars().collect();
    input_array.push('$');

    //Make the Data Array   [Suffix], num, [Rank Table]
    let mut data_table: Vec<(Vec<char>, usize, Vec<usize>)> = Vec::new();

    //TODO: Currently this generates the entire matrix, implement SA-SI to force it to only generate F and L column
    for i in 0..(input_array.len()) {
        let mut to_be_inserted = input_array.clone();
        let rank_table = Vec::new();

        if data_table.len() > 0{
            to_be_inserted = data_table.last().unwrap().0.to_vec();
            let last_elem = to_be_inserted.pop().unwrap();
            to_be_inserted.insert(0, last_elem);
        }
        data_table.push((to_be_inserted, i, rank_table))
    }

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

    for i in 0..(data_table.len()) {
        let suff = all_suffixes_ordered[i].0.clone();
        data_table[i].0 = vec![suff[0], *suff.last().unwrap()];
    }

    print!("\n\nF & L - SORTED");
    print!("\n  F  L    num");
    for elem in data_table {
        print!("\n  ");
        for inner_elem in elem.0 {
            print!("{}  ", inner_elem);
        }
        print!("   {}", elem.1);
    }
}

//TODO: GENERATE RANK TABLES
//TODO: MAKE 'QUERY' FUNCTION (FROM X IN F TO Y IN L)
//TODO: MAKE 'MATCH' FUNCTION THAT CALLS QUERY RECURSIVELY
//TODO: GENERATE F & L MORE EFFICIENTLY
//TODO: MAKE SKIPS IN RANKT TABLES
//TODO: MAKE SKIPS IN NUMBERING TABLE
//TODO: WRITE ABOUT IT
