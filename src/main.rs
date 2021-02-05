fn main() {
    let input_string = "abaaba";
    println!("  === INPUT IS \"{}\" ===", input_string);

    let mut input_array: Vec<_> = input_string.chars().collect();
    input_array.push('$');

    //make First and Last column of BWT
    let mut bwt_first: Vec<char> = Vec::new();
    let mut bwt_last: Vec<char> = Vec::new();
    let mut bwt_num: Vec<usize> = Vec::new();
    
    let mut all_suffixes_unordered: Vec<Vec<char>> = Vec::new();
    for i in 0..(input_array.len()) {
        let mut to_be_inserted = input_array.clone();
        if all_suffixes_unordered.len() > 0{
            to_be_inserted = all_suffixes_unordered.last().unwrap().to_vec();
            let last_elem = to_be_inserted.pop().unwrap();
            to_be_inserted.insert(0, last_elem)
        }
        all_suffixes_unordered.push(to_be_inserted)
    }

    for elem in all_suffixes_unordered {
        for inner_elem in elem {
            print!("{} ", inner_elem)
        }
        print!("\n")
    }
}
