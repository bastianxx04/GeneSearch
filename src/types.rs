use std::fmt::{Display, Formatter};

use crate::{bwt, ALPHABET};

pub type SuffixArray = Vec<usize>;

pub type CTable = Vec<usize>;

pub struct OTable<'a> {
    array: Vec<usize>,
    string: &'a [u8],
    suffix_array: &'a [usize],
}

impl<'a> OTable<'a> {
    pub fn new(string: &'a [u8], suffix_array: &'a [usize]) -> Self {
        let array_len = (string.len() + 1) * ALPHABET.len();
        OTable {
            array: vec![0; array_len],
            string,
            suffix_array,
        }
    }

    fn index(&self, a: u8, i: usize) -> Option<usize> {
        let a = a as usize;
        let (rows, cols) = self.shape();
        if a < rows && i < cols {
            Some(a * (self.string.len() + 1) + i)
        } else {
            None
        }
    }

    pub fn get(&self, a: u8, i: usize) -> usize {
        match self.index(a, i) {
            Some(idx) => self.array[idx],
            None => panic!(
                "GET index out of bounds ({}, {}) for shape {:?}",
                a,
                i,
                self.shape()
            ),
        }
    }

    pub fn set(&mut self, a: u8, i: usize, v: usize) {
        match self.index(a, i) {
            Some(idx) => {
                self.array[idx] = v;
            }
            None => panic!(
                "SET index out of bounds ({}, {}) for shape {:?}",
                a,
                i,
                self.shape()
            ),
        }
    }

    pub fn shape(&self) -> (usize, usize) {
        (ALPHABET.len(), self.string.len() + 1)
    }
}

impl<'a> Display for OTable<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Header row
        write!(f, "{:>3}", ' ')?;
        write!(f, "{:>3}", ' ')?;

        for i in 0..self.suffix_array.len() {
            let j = bwt(self.string, self.suffix_array, i);
            let c = ALPHABET[j as usize];
            write!(f, "{:>3}", c)?;
        }
        write!(f, "\n")?;

        // Data rows
        for (i, c) in ALPHABET.iter().enumerate() {
            write!(f, "{:>3}", c)?;
            for j in 0..(self.string.len() + 1) {
                let v = self.get(i as u8, j);
                write!(f, "{:>3}", v)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

pub type DTable = Vec<usize>;

#[cfg(test)]
mod tests {
    use crate::{suffix_array_construction::construct_suffix_array_naive, util::string_to_ints};

    use super::OTable;

    #[test]
    fn test_1() {
        let reference = string_to_ints("ACG$");
        let suffix_array = construct_suffix_array_naive(&reference);
        let mut o_table = OTable::new(&reference, &suffix_array);
        o_table.set(2, 3, 1);
        println!("{}", o_table);
    }
}
