use crate::suffix_array::SuffixArray;

/// Construct a suffix array
trait SuffixArrayConstructor {

    fn construct_suffix_array(input_string: &[char]) -> SuffixArray;
}

pub fn construct_suffix_array_naive(input_array: &Vec<char>) -> SuffixArray {
    let mut temp_data_table: Vec<(Vec<char>, usize)> = Vec::new();
    for i in 0..(input_array.len()) {
        let mut to_be_inserted = input_array.to_owned();

        if !temp_data_table.is_empty() {
            to_be_inserted = temp_data_table.last().unwrap().0.clone();
            to_be_inserted.remove(0);
        }
        temp_data_table.push((to_be_inserted, i));
    }

    temp_data_table.sort();

    let array = temp_data_table
        .iter()
        .map(|elem| elem.1)
        .collect();

    SuffixArray {
        array,
        string: input_array,
    }
}