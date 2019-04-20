use std::cmp::min;
use std::collections::{HashMap, HashSet};
use crate::graphemes_struct::Graphemes;
use len_trait::len::{Len};
use itertools::Itertools;
use std::ops::Index;
use push_trait::base::Push;

pub mod graphemes_struct;

type Coordinate = (usize, usize);

/// Calculates the levenshtein distance between two words
///
/// # Arguments
/// * `graphemes1` - Graphemes to compare with `graphemes2`
/// * `graphemes2` - Graphemes to compare with `graphemes1`
/// * `sub_cost` - Cost of substituting a character with another
///
/// # Example
/// ```
/// use nlp::levenshtein_distance;
/// use nlp::graphemes_struct::Graphemes;
/// assert_eq!(levenshtein_distance(&Graphemes::new("book"), &Graphemes::new("back"), 1), 2);
/// assert_eq!(levenshtein_distance(&Graphemes::new("back"), &Graphemes::new("book"), 1), 2);
/// assert_eq!(levenshtein_distance(&Graphemes::new("kitten"), &Graphemes::new("sitting"), 1), 3);
/// ```
pub fn levenshtein_distance<'a, T, U>(graphemes1 : &T, graphemes2: &T, sub_cost : usize) -> usize
    where T : Len + Index<usize, Output = U>, U: PartialEq + 'a {
    levenshtein_distance_recurrence_matrix(graphemes1, graphemes2, sub_cost)[graphemes1.len()][graphemes2.len()]
}

/// Returns the backtraced path as a vector of coordinates (row, col) from the levenshtein distance cost matrix
/// starting at `(0, 0)`
///
/// # Arguments
/// * `graphemes1` - Graphemes to compare with `graphemes2`
/// * `graphemes2` - Graphemes to compare with `graphemes1`
/// * `sub_cost` - Cost of substituting a character with another
///
/// # Example
/// ```
/// use nlp::alignment_path;
/// use nlp::graphemes_struct::Graphemes;
///
/// alignment_path(&Graphemes::new("dog"), &Graphemes::new("woof"), 1);
/// // returns [(0, 0), (1, 1), (2, 2), (3, 3), (3, 4)]
/// ```
pub fn alignment_path<'a, T, U>(graphemes1 : &T, graphemes2: &T, sub_cost : usize) -> Vec<Coordinate>
    where T : Len + Index<usize, Output = U>, U : PartialEq + 'a {
    let mat = alignment_matrix(graphemes1, graphemes2, sub_cost);
    let mut path = backtrace_alignment_matrix((graphemes1.len(), graphemes2.len()), mat);
    path.reverse();
    path
}

/// Returns an alignment of two strings as an array of two graphemes
/// # Arguments
/// * `graphemes1` - Graphemes to compare with `graphemes2`
/// * `graphemes2` - Graphemes to compare with `graphemes1`
/// * `sub_cost` - Cost of substituting a character with another
/// * `ins_del_char` - &str for indicating insertion/deletion
///
/// # Example
/// ```
/// use nlp::alignment_strings;
/// use nlp::graphemes_struct::Graphemes;
/// let intention = Graphemes::new("intention");
/// let execution = Graphemes::new("execution");
/// let strings = alignment_strings(&intention, &execution, 1, " ");
/// // strings contains
/// // 0. inten tion
/// // 1. ex ecution
/// ```
pub fn alignment_strings<'a, T, U>(graphemes1 : &T, graphemes2 : &T, sub_cost : usize, ins_del_char : U) -> [T; 2]
    where T : 'a + Default + Len + Push<U> + Index<usize, Output = U>, U : PartialEq + Clone + 'a{
    let path = alignment_path(graphemes1, graphemes2, sub_cost);
    if path.is_empty() {
        return [T::default(), T::default()];
    }
    let mut align_graphemes1 = T::default();
    let mut align_graphemes2 = T::default();

    let mut path_iter = path.iter();
    let mut prev_coord = *path_iter.next().unwrap(); // handled by the if case
    for &(row, col) in path_iter {
        if row != 0 && row - 1 == prev_coord.0 && col != 0 && col - 1 == prev_coord.1 {
            align_graphemes1.push(graphemes1[row-1].clone());
            align_graphemes2.push(graphemes2[col-1].clone());
        } else if row == prev_coord.0 && col != 0 && col - 1 == prev_coord.1 {
            align_graphemes1.push(ins_del_char.clone());
            align_graphemes2.push(graphemes2[col-1].clone());
        }
        else if row != 0 && row - 1 == prev_coord.0 && col == prev_coord.1 {
            align_graphemes1.push(graphemes1[row-1].clone());
            align_graphemes2.push(ins_del_char.clone());
        } else {
            panic!();
        }
        prev_coord = (row, col);
    }
    [align_graphemes1, align_graphemes2]
}

/// Segments a sentence with space using the max match algorithm
/// # Arguments
/// * `sentence` - Sentence composed of words unseperated to be segmented
/// * `dictionary` - HashSet containing words for matching possible words in the sentence for segmentation
///
/// # Example
/// ```
/// use nlp::max_match;
/// use nlp::graphemes_struct::Graphemes;
/// use std::collections::HashSet;
/// let mut dictionary : HashSet<Graphemes> = HashSet::new();
///        dictionary.insert(Graphemes::new("他"));
///        dictionary.insert(Graphemes::new("特别"));
///        dictionary.insert(Graphemes::new("喜欢"));
///        dictionary.insert(Graphemes::new("北京烤鸭"));
/// let sentence = max_match(&Graphemes::new("他特别喜欢北京烤鸭"), &chinese_dictionary);
/// assert_eq!(&sentence, &Graphemes::new("他 特别 喜欢 北京烤鸭"));
/// ```
pub fn max_match<'a>(sentence : &Graphemes<'a>, dictionary : &HashSet<Graphemes>) -> Graphemes<'a> {
    if sentence.is_empty() {
        return Graphemes::new("");
    }
    for i in (1..sentence.len()+1).rev() {
        let mut first_word = sentence.slice(0,i);
        let remainder = sentence.slice(i, sentence.len());
        if dictionary.contains(&first_word) {
            if !remainder.is_empty() {
                first_word.push(" ");
            }
            first_word.append(max_match(&remainder, dictionary));
            return first_word;
        }
    }
    let mut first_word = sentence.slice(0,1);
    let remainder = sentence.slice(1, sentence.len());

    if !remainder.is_empty() {
        first_word.push(" ");
    }
    first_word.append(max_match(&remainder, dictionary));
    return first_word;
}

//pub fn word_error_rate(sentence1 : &Graphemes, sentence2 : &Graphemes) -> usize {
//    let lev_distance = levenshtein_distance(sentence1, sentence2, 1);
//
//}

fn levenshtein_distance_recurrence_matrix<'a, T, U>(graphemes1 : &T, graphemes2 : &T, sub_cost : usize) -> Vec<Vec<usize>>
    where T : Len + Index<usize, Output = U>, U : PartialEq + 'a {
    let num_rows = graphemes1.len() + 1;
    let num_cols = graphemes2.len() + 1;
    let mut recurrence_matrix : Vec<Vec<usize>> = vec![vec![0; num_cols]; num_rows];
    // graphemes1 → row
    // graphemes2 → column
    for row in 1..num_rows {
        recurrence_matrix[row][0] = row;
    }
    for col in 1..num_cols {
        recurrence_matrix[0][col] = col;
    }

    for (row, col) in (1..num_rows).cartesian_product(1..num_cols) {
        recurrence_matrix[row][col] = min(min(
            recurrence_matrix[row-1][col]+1,
            recurrence_matrix[row][col-1]+1
        ),  recurrence_matrix[row-1][col-1] + if graphemes1[row-1] == graphemes2[col-1] {0} else {sub_cost})
    }
    recurrence_matrix
}


fn backtrace_alignment_matrix<'a>(start_coord : Coordinate, backtrace : HashMap<Coordinate, Coordinate>) -> Vec<Coordinate>{
    let mut path  = vec![];
    let mut backtracing_coord = start_coord;
    while let Some(&next_coord) = backtrace.get(&backtracing_coord) {
        path.push(backtracing_coord);
        backtracing_coord = next_coord;
    }
    path.push(backtracing_coord);
    path
}

fn alignment_matrix<'a, T, U>(graphemes1 : &T, graphemes2 : &T, sub_cost : usize) -> HashMap<Coordinate, Coordinate>
    where T : Len + Index<usize, Output = U>, U : PartialEq + 'a {
    let num_rows = graphemes1.len() + 1;
    let num_cols = graphemes2.len() + 1;
    let mut backtrace : HashMap<Coordinate, Coordinate> = HashMap::new();
    let mut recurrence_matrix : Vec<Vec<usize>> = vec![vec![0; num_cols]; num_rows];
    // graphemes1 → row
    // graphemes2 → column
    for row in 1..num_rows {
        recurrence_matrix[row][0] = row;
        backtrace.insert((row, 0), (row-1, 0));
    }
    for col in 1..num_cols {
        recurrence_matrix[0][col] = col;
        backtrace.insert((0, col), (0, col-1));
    }

    for (row, col) in (1..num_rows).cartesian_product(1..num_cols) {
        let mut min_distance = recurrence_matrix[row][col-1] + 1;
        let mut min_coordinate = (row, col-1);
        let current_del_cost = recurrence_matrix[row-1][col] + 1;
        if current_del_cost < min_distance {
            min_distance = current_del_cost;
            min_coordinate = (row-1, col);
        }
        let current_sub_cost = recurrence_matrix[row-1][col-1] + if graphemes1[row-1] == graphemes2[col-1] {0} else {sub_cost};
        if current_sub_cost < min_distance {
            min_distance = current_sub_cost;
            min_coordinate = (row-1, col-1);
        }

        recurrence_matrix[row][col] = min_distance;
        backtrace.insert((row, col), min_coordinate);
    }
    backtrace
}

#[cfg(test)]
mod test_cases {
    use super::*;

    #[test]
    fn edit_distance_basic_test() {
        // empty string
        assert_eq!(levenshtein_distance(&Graphemes::new(""), &Graphemes::new(""), 1), 0);
        // empty string symmetry
        assert_eq!(levenshtein_distance(&Graphemes::new(""), &Graphemes::new("a"), 1), 1);
        assert_eq!(levenshtein_distance(&Graphemes::new("a"), &Graphemes::new(""), 1), 1);

        assert_eq!(levenshtein_distance(&Graphemes::new("a"), &Graphemes::new("a"), 1), 0);
        assert_eq!(levenshtein_distance(&Graphemes::new("a"), &Graphemes::new("b"), 1), 1);
        assert_eq!(levenshtein_distance(&Graphemes::new("a"), &Graphemes::new("b"), 2), 2);
        assert_eq!(levenshtein_distance(&Graphemes::new("ab"), &Graphemes::new("a"), 1), 1);
        assert_eq!(levenshtein_distance(&Graphemes::new("a"), &Graphemes::new("ab"), 1), 1);
    }

    #[test]
    fn edit_distance_vec_of_graphemes_test() {
        assert_eq!(levenshtein_distance(&vec![Graphemes::new("")]
                                        , &vec![Graphemes::new(""),], 1), 0);
        assert_eq!(levenshtein_distance(&vec![Graphemes::new("hello"), Graphemes::new("world")]
                                        , &vec![Graphemes::new("bye"), Graphemes::new("bye")], 1), 2);
        assert_eq!(levenshtein_distance(&vec![Graphemes::new("hello")]
                                        , &vec![Graphemes::new("bye"), Graphemes::new("bye")], 2), 3);
        assert_eq!(levenshtein_distance(&vec![Graphemes::new("hello"), Graphemes::new("world")]
                                        , &vec![Graphemes::new("bye")], 2), 3);
    }

    #[test]
    fn edit_distance_example_test() {
        assert_eq!(levenshtein_distance(&Graphemes::new("book"), &Graphemes::new("back"), 1), 2);
        assert_eq!(levenshtein_distance(&Graphemes::new("back"), &Graphemes::new("book"), 1), 2);
        assert_eq!(levenshtein_distance(&Graphemes::new("kitten"), &Graphemes::new("sitting"), 1), 3);
        assert_eq!(levenshtein_distance(&Graphemes::new("sitting"), &Graphemes::new("kitten"), 1), 3);
        assert_eq!(levenshtein_distance(&Graphemes::new("longstring"), &Graphemes::new("short"), 1), 9);
        assert_eq!(levenshtein_distance(&Graphemes::new("short"), &Graphemes::new("longstring"), 1), 9);
        assert_eq!(levenshtein_distance(&Graphemes::new("superman"), &Graphemes::new("batman"), 1), 5);
        assert_eq!(levenshtein_distance(&Graphemes::new("batman"), &Graphemes::new("superman"), 1), 5);
        assert_eq!(levenshtein_distance(&Graphemes::new(""), &Graphemes::new("aaaaaaaaaaaaaaaaa"), 1), 17);
        assert_eq!(levenshtein_distance(&Graphemes::new("aaaaaaaaaaaaaaaaa"), &Graphemes::new(""), 1), 17);
    }

    #[test]
    fn edit_distance_chinese_test() {
        assert_eq!(levenshtein_distance(&Graphemes::new("己所不欲勿施于人"), &Graphemes::new("back"), 1), 8);
        assert_eq!(levenshtein_distance(&Graphemes::new("back"), &Graphemes::new("己所不欲勿施于人"), 1), 8);
        assert_eq!(levenshtein_distance(&Graphemes::new("己所不欲勿施于人"), &Graphemes::new("不患人之不己知患不知人也"), 1), 10);
        assert_eq!(levenshtein_distance(&Graphemes::new("不患人之不己知患不知人也"), &Graphemes::new("己所不欲勿施于人"), 1), 10);
    }

    fn calculate_edit_distance_from_alignment(graphemes1 : &Graphemes, graphemes2 : &Graphemes, sub_cost : usize, ins_del_char : &str) -> usize {
        let alignments = alignment_strings(graphemes1, graphemes2, sub_cost, ins_del_char);
        assert_eq!(alignments[0].len(), alignments[1].len());
        let mut edit_distance = 0;
        for i in 0..alignments[0].len() {
            if alignments[0][i] == " " || alignments[1][i] == " " {
                edit_distance += 1;
            } else if alignments[0][i] != alignments[1][i] {
                edit_distance += sub_cost;
            }
        }
        edit_distance
    }

    #[test]
    fn alignment_path_basic_test() {
        assert_eq!(calculate_edit_distance_from_alignment(
            &Graphemes::new(""), &Graphemes::new(""), 2, " "), 0);
        assert_eq!(calculate_edit_distance_from_alignment(
            &Graphemes::new(""), &Graphemes::new("a"), 2, " "), 1);
        assert_eq!(calculate_edit_distance_from_alignment(
            &Graphemes::new("a"), &Graphemes::new(""), 2, " "), 1);
        assert_eq!(calculate_edit_distance_from_alignment(
            &Graphemes::new(""), &Graphemes::new("aa"), 2, " "), 2);
        assert_eq!(calculate_edit_distance_from_alignment(
            &Graphemes::new("aa"), &Graphemes::new(""), 2, " "), 2);
        assert_eq!(calculate_edit_distance_from_alignment(
            &Graphemes::new("a"), &Graphemes::new("b"), 2, " "), 2);
    }

    #[test]
    fn alignment_path_example_test() {
        assert_eq!(calculate_edit_distance_from_alignment(
            &Graphemes::new("book"), &Graphemes::new("back"), 1, " "), 2);
        assert_eq!(calculate_edit_distance_from_alignment(
            &Graphemes::new("back"), &Graphemes::new("book"), 1, " "), 2);
        assert_eq!(calculate_edit_distance_from_alignment(
            &Graphemes::new("kitten"), &Graphemes::new("sitting"), 1, " "), 3);
        assert_eq!(calculate_edit_distance_from_alignment(
            &Graphemes::new("sitting"), &Graphemes::new("kitten"), 1, " "), 3);
        assert_eq!(calculate_edit_distance_from_alignment(
            &Graphemes::new("longstring"), &Graphemes::new("short"), 1, " "), 9);
        assert_eq!(calculate_edit_distance_from_alignment(
            &Graphemes::new("short"), &Graphemes::new("longstring"), 1, " "), 9);
        assert_eq!(calculate_edit_distance_from_alignment(
            &Graphemes::new("superman"), &Graphemes::new("batman"), 1, " "), 5);
        assert_eq!(calculate_edit_distance_from_alignment(
            &Graphemes::new("batman"), &Graphemes::new("superman"), 1, " "), 5);
        assert_eq!(calculate_edit_distance_from_alignment(
            &Graphemes::new(""), &Graphemes::new("aaaaaaaaaaaaaaaaa"), 1, " "), 17);
        assert_eq!(calculate_edit_distance_from_alignment(
            &Graphemes::new("aaaaaaaaaaaaaaaaa"), &Graphemes::new(""), 1, " "), 17);
    }

    fn chinese_dictionary() -> HashSet<Graphemes<'static>> {
        let mut dictionary : HashSet<Graphemes> = HashSet::new();
        dictionary.insert(Graphemes::new("他"));
        dictionary.insert(Graphemes::new("特别"));
        dictionary.insert(Graphemes::new("喜欢"));
        dictionary.insert(Graphemes::new("北京烤鸭"));
        dictionary
    }

    #[test]
    fn max_match_test() {
        let chinese_dictionary = chinese_dictionary();
        let empty_sentence : Graphemes = max_match(&Graphemes::new(""), &chinese_dictionary);
        assert!(empty_sentence.is_empty());
        let sentence = max_match(&Graphemes::new("他特别喜欢北京烤鸭"), &chinese_dictionary);
        assert_eq!(&sentence, &Graphemes::new("他 特别 喜欢 北京烤鸭"));
        let another_sentence = max_match(&Graphemes::new("english"), &chinese_dictionary);
        assert_eq!(&another_sentence, &Graphemes::new("e n g l i s h"));
    }
}