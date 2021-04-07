use crate::{bwt, ALPHABET};
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

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

    fn calc_index(&self, a: u8, i: usize) -> Option<usize> {
        let a = a as usize;
        let (rows, cols) = self.shape();
        if a < rows && i < cols {
            Some(a * (self.string.len() + 1) + i)
        } else {
            None
        }
    }

    pub fn shape(&self) -> (usize, usize) {
        (ALPHABET.len(), self.string.len() + 1)
    }
}

impl Index<(u8, usize)> for OTable<'_> {
    type Output = usize;

    fn index(&self, index: (u8, usize)) -> &usize {
        let (a, i) = index;
        match self.calc_index(a, i) {
            Some(idx) => &self.array[idx],
            None => panic!(
                "GET index out of bounds ({}, {}) for shape {:?}",
                a,
                i,
                self.shape()
            ),
        }
    }
}

impl IndexMut<(u8, usize)> for OTable<'_> {
    fn index_mut(&mut self, index: (u8, usize)) -> &mut usize {
        let (a, i) = index;
        match self.calc_index(a, i) {
            Some(idx) => self.array.get_mut(idx).expect("index was out of bounds"),
            None => panic!(
                "SET index out of bounds ({}, {}) for shape {:?}",
                a,
                i,
                self.shape()
            ),
        }
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
        writeln!(f,)?;

        // Data rows
        for (i, c) in ALPHABET.iter().enumerate() {
            write!(f, "{:>3}", c)?;
            for j in 0..(self.string.len() + 1) {
                write!(f, "{:>3}", self[(i as u8, j)])?;
            }
            writeln!(f,)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{suffix_array_construction::suffix_array_induced_sort, util::remap_string};

    use super::OTable;

    #[test]
    fn test_1() {
        let reference = remap_string("ACG$");
        let suffix_array = suffix_array_induced_sort(&reference);
        let mut o_table = OTable::new(&reference, &suffix_array);
        o_table[(2, 3)] = 1;
        println!("{}", o_table);
    }
}
