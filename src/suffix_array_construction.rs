/// Construct a suffix array
trait SuffixArrayConstructor {
    fn construct_suffix_array(input_string: Vec<char>) -> Vec<usize>;
}

pub fn construct_suffix_array_naive(input_array: &Vec<char>) -> Vec<usize> {
    let mut temp_data_table: Vec<(Vec<char>, usize)> = Vec::new();
    for i in 0..(input_array.len()) {
        let mut to_be_inserted = input_array.clone();

        if temp_data_table.len() > 0 {
            to_be_inserted = temp_data_table.last().unwrap().0.to_vec();
            to_be_inserted.remove(0);
        }
        temp_data_table.push((to_be_inserted, i));
    }

    temp_data_table.sort();

    return temp_data_table
        .iter()
        .map(|elem| elem.1)
        .collect();
}