pub struct SuffixArray<'a> {
    pub string: &'a Vec<u8>,
    pub array: Vec<usize>,
}

pub type CTable = Vec<usize>;
pub type OTable = Vec<Vec<usize>>;
pub type DTable = Vec<usize>;
