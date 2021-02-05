fn main() {
    let input_string = "abaaba";
    println!("  === INPUT IS \"{}\" ===", input_string);

    let mut input_array: Vec<_> = input_string.chars().collect();
    input_array.push('$');

    //make First and Last column of BWT
    let mut bwt_first: Vec<char> = Vec::new();
    let mut bwt_last: Vec<char> = Vec::new();
    let mut bwt_num: Vec<usize> = Vec::new();
    
    let mut all_suffixes_unordered: Vec<(Vec<char>, usize)> = Vec::new();
    for i in 0..(input_array.len()) {
        let mut to_be_inserted = input_array.clone();
        if all_suffixes_unordered.len() > 0{
            to_be_inserted = all_suffixes_unordered.last().unwrap().0.to_vec();
            let last_elem = to_be_inserted.pop().unwrap();
            to_be_inserted.insert(0, last_elem)
        }
        all_suffixes_unordered.push((to_be_inserted, i))
    }

    let mut all_suffixes_ordered: Vec<(Vec<char>, usize)> = all_suffixes_unordered.clone();

    print!("GENERATED BWT MATRIX - UNSORTED");
    for elem in all_suffixes_unordered {
        print!("\n    ");
        for inner_elem in elem.0 {
            print!("{} ", inner_elem);
        }
        print!("    {}", elem.1);
    }
    
    all_suffixes_ordered.sort();
    print!("\n\nGENERATED BWT MATRIX - SORTED");
    for elem in all_suffixes_ordered {
        print!("\n    ");
        for inner_elem in elem.0 {
            print!("{} ", inner_elem);
        }
        print!("    {}", elem.1);
    }
}
