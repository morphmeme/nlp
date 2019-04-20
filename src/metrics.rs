pub use self::metrics::levenshtein_distance;
pub use self::metrics::word_error_rate;
pub use self::metrics::word_accuracy;

mod metrics {
    use itertools::Itertools;
    use len_trait::len::Len;
    use std::ops::Index;
    use std::cmp::min;
    use crate::graphemes_struct::Graphemes;

    /// Calculates the levenshtein distance between two words
    ///
    /// # Arguments
    /// * `graphemes1` - Graphemes to compare with `graphemes2`
    /// * `graphemes2` - Graphemes to compare with `graphemes1`
    /// * `sub_cost` - Cost of substituting a character with another
    ///
    /// # Example
    /// ```
    /// use nlp::metrics::levenshtein_distance;
    /// use nlp::graphemes_struct::Graphemes;
    /// assert_eq!(levenshtein_distance(&Graphemes::new("book"), &Graphemes::new("back"), 1), 2);
    /// assert_eq!(levenshtein_distance(&Graphemes::new("back"), &Graphemes::new("book"), 1), 2);
    /// assert_eq!(levenshtein_distance(&Graphemes::new("kitten"), &Graphemes::new("sitting"), 1), 3);
    /// ```
    pub fn levenshtein_distance<'a, T, U>(graphemes1 : &T, graphemes2: &T, sub_cost : usize) -> usize
        where T : Len + Index<usize, Output = U>, U: PartialEq + 'a {
        levenshtein_distance_recurrence_matrix(graphemes1, graphemes2, sub_cost)[graphemes1.len()][graphemes2.len()]
    }


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

    /// Calculates the word error rate (word insertions + deletions + substitutions) / (length of the correct sentence)
    ///
    /// # Arguments
    /// * `actual_sentence` - actual sentence
    /// * `predict_sentence` - predicted sentence
    ///
    /// # Example
    /// ```
    /// use nlp::metrics::word_error_rate;
    /// use nlp::graphemes_struct::Graphemes;
    /// use nlp::max_match;
    /// use std::collections::HashSet;
    /// let mut dictionary : HashSet<Graphemes> = HashSet::new();
    /// dictionary.insert(Graphemes::new("we"));
    /// dictionary.insert(Graphemes::new("canon"));
    /// dictionary.insert(Graphemes::new("see"));
    /// dictionary.insert(Graphemes::new("ash"));
    /// dictionary.insert(Graphemes::new("ort"));
    /// dictionary.insert(Graphemes::new("distance"));
    /// dictionary.insert(Graphemes::new("ahead"));
    /// let predicted_sentence = max_match(&Graphemes::new("wecanonlyseeashortdistanceahead"), &dictionary);
    /// let actual_sentence = Graphemes::new("we can only see a short distance ahead");
    /// assert_eq!(word_error_rate(&actual_sentence, &predicted_sentence),0.625);
    /// ```
    pub fn word_error_rate(actual_sentence : &Graphemes, predict_sentence : &Graphemes) -> f64 {
        let actual_split_sentence = actual_sentence.split(" ");
        let lev_distance = levenshtein_distance(&actual_split_sentence, &predict_sentence.split(" "), 1);
        lev_distance as f64 / actual_split_sentence.len() as f64
    }

    /// Calculates the word accuracy 1 - (word insertions + deletions + substitutions) / (length of the correct sentence)
    ///
    /// # Arguments
    /// * `actual_sentence` - actual sentence
    /// * `predict_sentence` - predicted sentence
    ///
    /// # Example
    /// ```
    /// use nlp::metrics::word_accuracy;
    /// use nlp::graphemes_struct::Graphemes;
    /// use nlp::max_match;
    /// use std::collections::HashSet;
    /// let mut dictionary : HashSet<Graphemes> = HashSet::new();
    /// dictionary.insert(Graphemes::new("we"));
    /// dictionary.insert(Graphemes::new("canon"));
    /// dictionary.insert(Graphemes::new("see"));
    /// dictionary.insert(Graphemes::new("ash"));
    /// dictionary.insert(Graphemes::new("ort"));
    /// dictionary.insert(Graphemes::new("distance"));
    /// dictionary.insert(Graphemes::new("ahead"));
    /// let predicted_sentence = max_match(&Graphemes::new("wecanonlyseeashortdistanceahead"), &dictionary);
    /// let actual_sentence = Graphemes::new("we can only see a short distance ahead");
    /// assert_eq!(word_accuracy(&actual_sentence, &predicted_sentence),0.375);
    /// ```
    pub fn word_accuracy(actual_sentence : &Graphemes, predict_sentence : &Graphemes) -> f64 {
        1.0 - word_error_rate(actual_sentence, predict_sentence)
    }
}

#[cfg(tests)]
mod test_cases {
    use crate::metrics::{levenshtein_distance, word_error_rate};
    use crate::graphemes_struct::Graphemes;
    use crate::max_match;
    use std::collections::HashSet;

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

    #[test]
    fn word_error_rate_test() {
        let mut dictionary : HashSet<Graphemes> = HashSet::new();
        dictionary.insert(Graphemes::new("we"));
        dictionary.insert(Graphemes::new("canon"));
        dictionary.insert(Graphemes::new("see"));
        dictionary.insert(Graphemes::new("ash"));
        dictionary.insert(Graphemes::new("ort"));
        dictionary.insert(Graphemes::new("distance"));
        dictionary.insert(Graphemes::new("ahead"));
        let predicted_sentence = max_match(&Graphemes::new("wecanonlyseeashortdistanceahead"), &dictionary);
        let actual_sentence = Graphemes::new("we can only see a short distance ahead");
        assert_eq!(word_error_rate(&actual_sentence, &predicted_sentence),0.625);
        assert_eq!(word_error_rate(&actual_sentence, &actual_sentence),0.0)
    }
}