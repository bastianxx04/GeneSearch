use crate::{bwt, ALPHABET};
use std::io::{BufReader, Write};
use std::{
    fmt::{Display, Formatter},
    fs::File,
    path::Path,
};

pub struct OTable<'a> {
    array: Vec<usize>,
    spacing: usize,
    sentinel: usize,
    string: &'a [u8],
    suffix_array: &'a [usize],
}

impl<'a> OTable<'a> {
    /// Allocate and generate an O-table.
    pub fn new(string: &'a [u8], suffix_array: &'a [usize], spacing: usize) -> Self {
        let array_len = ((string.len() / spacing) + 1) * (ALPHABET.len() - 1);
        let mut o_table = OTable {
            array: vec![0; array_len],
            spacing,
            sentinel: usize::MAX,
            string,
            suffix_array,
        };

        // Fill O-table
        let (_, cols) = o_table.internal_shape();
        let mut counter = vec![0; 4];
        for i in 1..cols {
            let c = bwt(o_table.string, o_table.suffix_array, i - 1);
            if c == 0 {
                o_table.sentinel = i;
            } else {
                counter[(c - 1) as usize] += 1;
            }

            if i % spacing == 0 {
                for (a, &c) in counter.iter().enumerate() {
                    o_table.set((a + 1) as u8, i, c);
                }
            }
        }

        o_table
    }

    // Reads an O-table from a given file
    pub fn from_file(
        filename: &str,
        string: &'a [u8],
        suffix_array: &'a [usize],
        spacing: usize,
    ) -> Self {
        let mut otable_path = Path::new("resources/otable/").join(filename);
        otable_path.set_extension(spacing.to_string());
        match File::open(&otable_path) {
            Ok(f) => {
                let buf_reader = BufReader::new(f);
                let mut decoded: Vec<usize> = bincode::deserialize_from(buf_reader).unwrap();
                let sentinel = decoded.pop().unwrap(); // Tjek om decoded size ændrer sig
                OTable {
                    array: decoded,
                    spacing,
                    sentinel,
                    string: &string,
                    suffix_array: &suffix_array,
                }
            }
            Err(_) => {
                // File doesn't exist, generate it
                let o_table = OTable::new(&string, &suffix_array, spacing);
                let mut array = o_table.array.clone();
                array.push(o_table.sentinel);
                let bytes: Vec<u8> = bincode::serialize(&array).unwrap();
                let mut file = File::create(&otable_path).unwrap();
                file.write_all(&bytes).unwrap();
                o_table
            }
        }
    }

    /// Calculates the index into the internal array.
    /// Returns a two values:
    /// - the first is the index into the internal array
    /// - the second is the remaining lines to count the character in
    fn calc_index(&self, a: u8, i: usize) -> (usize, usize) {
        let a = (a - 1) as usize;
        let (rows, cols) = self.internal_shape();

        if a >= rows || i >= cols {
            panic!(
                "Index out of bounds ({}, {}) for shape ({}, {})",
                a, i, rows, cols,
            )
        }
        let offset = (self.array.len() / (ALPHABET.len() - 1)) * a;

        (offset + i / self.spacing, i % self.spacing)
    }

    fn internal_shape(&self) -> (usize, usize) {
        (ALPHABET.len() - 1, self.string.len() + 1)
    }

    pub fn shape(&self) -> (usize, usize) {
        (ALPHABET.len(), self.string.len() + 1)
    }

    fn find_count(&self, from: usize, to: usize, character: u8) -> usize {
        let mut count = 0;

        for i in from..to {
            if character == bwt(self.string, self.suffix_array, i) {
                count += 1;
            }
        }
        count
    }

    pub fn get(&self, a: u8, i: usize) -> usize {
        if a == 0 {
            return (i >= self.sentinel) as usize;
        }
        match self.calc_index(a, i) {
            (idx, 0) => self.array[idx],
            (idx, _) => self.array[idx] + self.find_count(i - (i % self.spacing), i, a),
        }
    }

    fn set(&mut self, a: u8, i: usize, v: usize) {
        if a == 0 {
            return;
        }
        match self.calc_index(a, i) {
            (idx, 0) => self.array[idx] = v,
            _ => todo!(),
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
        read_genome, suffix_array_construction::suffix_array_induced_sort, util::remap_string,
        ALPHABET, HG38_1000,
    };
    use test::Bencher;

    #[test]
    fn test_o_table_shape() {
        let reference = remap_string("ACGTATCGTGACGGGCTATAGCGATGTCGATGC$");
        let sa = suffix_array_induced_sort(&reference);
        let o_table = OTable::new(&reference, &sa, 10);
        let (rows, cols) = o_table.shape();
        assert_eq!(rows, ALPHABET.len());
        assert_eq!(cols, reference.len() + 1);
    }

    #[test]
    fn test_o_table_get() {
        /* Full O-table for ACGTATCGTGACGGGCTATAGCGATGTCGATGC$
            0                             10                            20                            30
               C  G  $  T  T  T  G  G  G  T  G  A  A  T  G  T  C  C  T  A  G  G  C  C  T  C  A  C  G  G  A  G  A  A
        $ | 0  0  0  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1
        A | 0  0  0  0  0  0  0  0  0  0  0  0  1  2  2  2  2  2  2  2  3  3  3  3  3  3  3  4  4  4  4  5  5  6  7
        C | 0  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  2  3  3  3  3  3  4  5  5  6  6  7  7  7  7  7  7  7
        G | 0  0  1  1  1  1  1  2  3  4  4  5  5  5  5  6  6  6  6  6  6  7  8  8  8  8  8  8  8  9 10 10 11 11 11
        T | 0  0  0  0  1  2  3  3  3  3  4  4  4  4  5  5  6  6  6  7  7  7  7  7  7  8  8  8  8  8  8  8  8  8  8
        */
        let reference = remap_string("ACGTATCGTGACGGGCTATAGCGATGTCGATGC$");
        let sa = suffix_array_induced_sort(&reference);
        let o_table = OTable::new(&reference, &sa, 10);
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
        let reference = remap_string("ACGTATCGTGACGGGCTATAGCGATGTCGATGC$");
        let sa = suffix_array_induced_sort(&reference);
        let o_table = OTable::new(&reference, &sa, 10);
        assert_eq!((0, 0), o_table.calc_index(1, 0));
        assert_eq!((1, 3), o_table.calc_index(1, 13));
        assert_eq!((10, 0), o_table.calc_index(3, 20));
        assert_eq!((10, 1), o_table.calc_index(3, 21));
        assert_eq!((6, 9), o_table.calc_index(2, 29));
    }

    #[bench]
    fn bench_o_table_ref1000(b: &mut Bencher) {
        let genome_string = read_genome(HG38_1000).unwrap();
        let genome = remap_string(&genome_string);
        let suffix_array = suffix_array_induced_sort(&genome);
        b.iter(|| OTable::new(&genome, &suffix_array, 10));
    }
}
