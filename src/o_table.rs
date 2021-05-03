use crate::{bwt, ALPHABET};
use std::io::{BufReader, Write};
use std::{
    fmt::{Display, Formatter},
    fs::File,
    path::Path,
};

pub struct OTable<'a> {
    array: Vec<usize>,
    string: &'a [u8],
    suffix_array: &'a [usize],
}

impl<'a> OTable<'a> {
    /// Allocate and generate an O-table.
    pub fn new(string: &'a [u8], suffix_array: &'a [usize]) -> Self {
        let array_len = (string.len() + 1) * ALPHABET.len();
        let mut o_table = OTable {
            array: vec![0; array_len],
            string,
            suffix_array,
        };

        // Fill O-table
        let (_, cols) = o_table.shape();
        let mut counter = vec![0; 5];
        for i in 1..cols {
            let c = bwt(o_table.string, o_table.suffix_array, i - 1);
            counter[c as usize] += 1;
            for (a, &c) in counter.iter().enumerate() {
                o_table.set(a as u8, i, c);
            }
        }

        o_table
    }

    // Reads an O-table from a given file
    pub fn from_file(filename: &str, string: &'a [u8], suffix_array: &'a [usize]) -> Self {
        let otable_path = Path::new("resources/otable/").join(filename);
        match File::open(&otable_path) {
            Ok(f) => {
                let buf_reader = BufReader::new(f);
                let decoded: Vec<usize> = bincode::deserialize_from(buf_reader).unwrap();
                OTable {
                    array: decoded,
                    string: &string,
                    suffix_array: &suffix_array,
                }
            }
            Err(_) => {
                // File doesn't exist, generate it
                let o_table = OTable::new(&string, &suffix_array);
                let bytes: Vec<u8> = bincode::serialize(&o_table.array).unwrap();
                let mut file = File::create(&otable_path).unwrap();
                file.write_all(&bytes).unwrap();
                o_table
            }
        }
    }

    /// Calculates the index into the internal array.
    fn calc_index(&self, a: u8, i: usize) -> usize {
        let a = a as usize;
        let (rows, cols) = self.shape();

        if a >= rows || i >= cols {
            panic!(
                "Index out of bounds ({}, {}) for shape ({}, {})",
                a, i, rows, cols,
            )
        }

        a * cols + i
    }

    pub fn shape(&self) -> (usize, usize) {
        (ALPHABET.len(), self.string.len() + 1)
    }

    pub fn get(&self, a: u8, i: usize) -> usize {
        let idx = self.calc_index(a, i);
        self.array[idx]
    }

    fn set(&mut self, a: u8, i: usize, v: usize) {
        let idx = self.calc_index(a, i);
        self.array[idx] = v;
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
                write!(f, "{:>3}", self.get(i as u8, j))?;
            }
            writeln!(f,)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::OTable;
    use crate::{
        suffix_array_construction::suffix_array_induced_sort, util::remap_reference, ALPHABET,
    };

    #[test]
    fn test_o_table_shape() {
        let reference = remap_reference("ACGTATCGTGACGGGCTATAGCGATGTCGATGC");
        let sa = suffix_array_induced_sort(&reference);
        let o_table = OTable::new(&reference, &sa);
        let (rows, cols) = o_table.shape();
        assert_eq!(rows, ALPHABET.len());
        assert_eq!(cols, reference.len() + 1);
    }

    #[test]
    fn test_o_table_get() {
        /* Full O-table for ACGTATCGTGACGGGCTATAGCGATGTCGATGC
            0                             10                            20                            30
               C  G  $  T  T  T  G  G  G  T  G  A  A  T  G  T  C  C  T  A  G  G  C  C  T  C  A  C  G  G  A  G  A  A
        $ | 0  0  0  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1
        A | 0  0  0  0  0  0  0  0  0  0  0  0  1  2  2  2  2  2  2  2  3  3  3  3  3  3  3  4  4  4  4  5  5  6  7
        C | 0  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  2  3  3  3  3  3  4  5  5  6  6  7  7  7  7  7  7  7
        G | 0  0  1  1  1  1  1  2  3  4  4  5  5  5  5  6  6  6  6  6  6  7  8  8  8  8  8  8  8  9 10 10 11 11 11
        T | 0  0  0  0  1  2  3  3  3  3  4  4  4  4  5  5  6  6  6  7  7  7  7  7  7  8  8  8  8  8  8  8  8  8  8
        */
        let reference = remap_reference("ACGTATCGTGACGGGCTATAGCGATGTCGATGC");
        let sa = suffix_array_induced_sort(&reference);
        let o_table = OTable::new(&reference, &sa);
        println!("{:>6}{:>30}{:>30}{:>30}", 'x', 'x', 'x', 'x');
        println!("{}", o_table);
        assert_eq!(o_table.get(2, 1), 1);
        assert_eq!(o_table.get(0, 3), 1);
        assert_eq!(o_table.get(0, 34), 1);
        assert_eq!(o_table.get(2, 13), 1);
        assert_eq!(o_table.get(3, 13), 5);
        assert_eq!(o_table.get(1, 30), 4);
    }

    #[test]
    fn test_calc_index() {
        let reference = remap_reference("ACGTATCGTGACGGGCTATAGCGATGTCGATGC");
        let sa = suffix_array_induced_sort(&reference);
        let o_table = OTable::new(&reference, &sa);
        assert_eq!(0, o_table.calc_index(0, 0));
        assert_eq!(16, o_table.calc_index(0, 16));
        assert_eq!(35, o_table.calc_index(1, 0));
        assert_eq!(48, o_table.calc_index(1, 13));
        assert_eq!(99, o_table.calc_index(2, 29));
        assert_eq!(125, o_table.calc_index(3, 20));
        assert_eq!(126, o_table.calc_index(3, 21));
    }
}
