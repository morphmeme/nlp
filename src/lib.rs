use std::cmp::min;
use itertools::Itertools;

extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;
use std::ops::Deref;
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Debug)]
pub struct Graphemes<'a> {
    graphemes : Vec<&'a str>
}

impl<'a> Graphemes<'a> {
    pub fn new(string : &'a str) -> Graphemes<'a> {
        Graphemes {
            graphemes: UnicodeSegmentation::graphemes(string, true).collect::<Vec<&str>>()
        }
    }

    pub fn len(&self) -> usize {
        self.graphemes.len()
    }

}

impl<'a> Display for Graphemes<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.graphemes.concat())
    }
}

impl<'a> Deref for Graphemes<'a> {
    type Target = Vec<&'a str>;

    fn deref(&self) -> &Self::Target {
        &self.graphemes
    }
}


pub fn levenshtein_distance(str1 : Graphemes, str2: Graphemes) -> usize {
    if str1.len() == 0 {
        return str2.len()
    }
    if str2.len() == 0 {
        return str1.len()
    }
    levenshtein_distance_recurrence_matrix(&str1, &str2)[str1.len()][str2.len()]
}

fn levenshtein_distance_recurrence_matrix(str1 : &Vec<&str>, str2 : &Vec<&str>) -> Vec<Vec<usize>> {
    let num_rows = str1.len() + 1;
    let num_cols = str2.len() + 1;
    let mut recurrence_matrix : Vec<Vec<usize>> = vec![vec![0; num_cols]; num_rows];
    // str1 → row
    // str2 → column
    for row in 0..num_rows {
        recurrence_matrix[row][0] = row;
    }
    for col in 0..num_cols {
        recurrence_matrix[0][col] = col;
    }

    for (row, col) in (1..num_rows).cartesian_product(1..num_cols) {
        recurrence_matrix[row][col] = min(min(
            recurrence_matrix[row-1][col]+1,
            recurrence_matrix[row][col-1]+1
        ),  recurrence_matrix[row-1][col-1] + if str1[row-1] == str2[col-1] {0} else {1})
    }
    recurrence_matrix
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn edit_distance_basic_test() {
        // empty string
        assert_eq!(levenshtein_distance(Graphemes::new(""), Graphemes::new("")), 0);
        // empty string symmetry
        assert_eq!(levenshtein_distance(Graphemes::new(""), Graphemes::new("a")), 1);
        assert_eq!(levenshtein_distance(Graphemes::new("a"), Graphemes::new("")), 1);

        assert_eq!(levenshtein_distance(Graphemes::new("a"), Graphemes::new("a")), 0);
        assert_eq!(levenshtein_distance(Graphemes::new("ab"), Graphemes::new("a")), 1);
        assert_eq!(levenshtein_distance(Graphemes::new("a"), Graphemes::new("ab")), 1);
    }

    #[test]
    fn edit_distance_example_test() {
        assert_eq!(levenshtein_distance(Graphemes::new("book"), Graphemes::new("back")), 2);
        assert_eq!(levenshtein_distance(Graphemes::new("back"), Graphemes::new("book")), 2);
        assert_eq!(levenshtein_distance(Graphemes::new("kitten"), Graphemes::new("sitting")), 3);
        assert_eq!(levenshtein_distance(Graphemes::new("sitting"), Graphemes::new("kitten")), 3);
        assert_eq!(levenshtein_distance(Graphemes::new("longstring"), Graphemes::new("short")), 9);
        assert_eq!(levenshtein_distance(Graphemes::new("short"), Graphemes::new("longstring")), 9);
        assert_eq!(levenshtein_distance(Graphemes::new("superman"), Graphemes::new("batman")), 5);
        assert_eq!(levenshtein_distance(Graphemes::new("batman"), Graphemes::new("superman")), 5);
        assert_eq!(levenshtein_distance(Graphemes::new(""), Graphemes::new("aaaaaaaaaaaaaaaaa")), 17);
        assert_eq!(levenshtein_distance(Graphemes::new("aaaaaaaaaaaaaaaaa"), Graphemes::new("")), 17);
    }

    #[test]
    fn edit_distance_chinese_test() {
        assert_eq!(levenshtein_distance(Graphemes::new("己所不欲勿施于人"), Graphemes::new("back")), 8);
        assert_eq!(levenshtein_distance(Graphemes::new("back"), Graphemes::new("己所不欲勿施于人")), 8);
        assert_eq!(levenshtein_distance(Graphemes::new("己所不欲勿施于人"), Graphemes::new("不患人之不己知患不知人也")), 10);
        assert_eq!(levenshtein_distance(Graphemes::new("不患人之不己知患不知人也"), Graphemes::new("己所不欲勿施于人")), 10);
    }
}