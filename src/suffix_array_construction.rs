use crate::types::SuffixArray;

/// Construct a suffix array naively
pub fn construct_suffix_array_naive(input_array: &[u8]) -> SuffixArray {
    let mut temp_data_table: Vec<(Vec<u8>, usize)> = Vec::new();
    for i in 0..(input_array.len()) {
        let mut to_be_inserted = input_array.to_owned();

        if !temp_data_table.is_empty() {
            to_be_inserted = temp_data_table.last().unwrap().0.clone();
            to_be_inserted.remove(0);
        }
        temp_data_table.push((to_be_inserted, i));
    }

    temp_data_table.sort();

    temp_data_table.iter().map(|elem| elem.1).collect()
}
