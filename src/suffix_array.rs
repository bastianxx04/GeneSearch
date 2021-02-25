pub struct SuffixArray<'a> {
    pub string: &'a Vec<char>,
    pub array: Vec<usize>,
}