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
    /// assert_eq!(levenshtein_distance(&Graphemes::from("book"), &Graphemes::from("back"), 1), 2);
    /// assert_eq!(levenshtein_distance(&Graphemes::from("back"), &Graphemes::from("book"), 1), 2);
    /// assert_eq!(levenshtein_distance(&Graphemes::from("kitten"), &Graphemes::from("sitting"), 1), 3);
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
    /// dictionary.insert(Graphemes::from("we"));
    /// dictionary.insert(Graphemes::from("canon"));
    /// dictionary.insert(Graphemes::from("see"));
    /// dictionary.insert(Graphemes::from("ash"));
    /// dictionary.insert(Graphemes::from("ort"));
    /// dictionary.insert(Graphemes::from("distance"));
    /// dictionary.insert(Graphemes::from("ahead"));
    /// let predicted_sentence = max_match(&Graphemes::from("wecanonlyseeashortdistanceahead"), &dictionary);
    /// let actual_sentence = Graphemes::from("we can only see a short distance ahead");
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
    /// dictionary.insert(Graphemes::from("we"));
    /// dictionary.insert(Graphemes::from("canon"));
    /// dictionary.insert(Graphemes::from("see"));
    /// dictionary.insert(Graphemes::from("ash"));
    /// dictionary.insert(Graphemes::from("ort"));
    /// dictionary.insert(Graphemes::from("distance"));
    /// dictionary.insert(Graphemes::from("ahead"));
    /// let predicted_sentence = max_match(&Graphemes::from("wecanonlyseeashortdistanceahead"), &dictionary);
    /// let actual_sentence = Graphemes::from("we can only see a short distance ahead");
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
        assert_eq!(levenshtein_distance(&Graphemes::from(""), &Graphemes::from(""), 1), 0);
        // empty string symmetry
        assert_eq!(levenshtein_distance(&Graphemes::from(""), &Graphemes::from("a"), 1), 1);
        assert_eq!(levenshtein_distance(&Graphemes::from("a"), &Graphemes::from(""), 1), 1);

        assert_eq!(levenshtein_distance(&Graphemes::from("a"), &Graphemes::from("a"), 1), 0);
        assert_eq!(levenshtein_distance(&Graphemes::from("a"), &Graphemes::from("b"), 1), 1);
        assert_eq!(levenshtein_distance(&Graphemes::from("a"), &Graphemes::from("b"), 2), 2);
        assert_eq!(levenshtein_distance(&Graphemes::from("ab"), &Graphemes::from("a"), 1), 1);
        assert_eq!(levenshtein_distance(&Graphemes::from("a"), &Graphemes::from("ab"), 1), 1);
    }

    #[test]
    fn edit_distance_vec_of_graphemes_test() {
        assert_eq!(levenshtein_distance(&vec![Graphemes::from("")]
                                        , &vec![Graphemes::from(""),], 1), 0);
        assert_eq!(levenshtein_distance(&vec![Graphemes::from("hello"), Graphemes::from("world")]
                                        , &vec![Graphemes::from("bye"), Graphemes::from("bye")], 1), 2);
        assert_eq!(levenshtein_distance(&vec![Graphemes::from("hello")]
                                        , &vec![Graphemes::from("bye"), Graphemes::from("bye")], 2), 3);
        assert_eq!(levenshtein_distance(&vec![Graphemes::from("hello"), Graphemes::from("world")]
                                        , &vec![Graphemes::from("bye")], 2), 3);
    }

    #[test]
    fn edit_distance_example_test() {
        assert_eq!(levenshtein_distance(&Graphemes::from("book"), &Graphemes::from("back"), 1), 2);
        assert_eq!(levenshtein_distance(&Graphemes::from("back"), &Graphemes::from("book"), 1), 2);
        assert_eq!(levenshtein_distance(&Graphemes::from("kitten"), &Graphemes::from("sitting"), 1), 3);
        assert_eq!(levenshtein_distance(&Graphemes::from("sitting"), &Graphemes::from("kitten"), 1), 3);
        assert_eq!(levenshtein_distance(&Graphemes::from("longstring"), &Graphemes::from("short"), 1), 9);
        assert_eq!(levenshtein_distance(&Graphemes::from("short"), &Graphemes::from("longstring"), 1), 9);
        assert_eq!(levenshtein_distance(&Graphemes::from("superman"), &Graphemes::from("batman"), 1), 5);
        assert_eq!(levenshtein_distance(&Graphemes::from("batman"), &Graphemes::from("superman"), 1), 5);
        assert_eq!(levenshtein_distance(&Graphemes::from(""), &Graphemes::from("aaaaaaaaaaaaaaaaa"), 1), 17);
        assert_eq!(levenshtein_distance(&Graphemes::from("aaaaaaaaaaaaaaaaa"), &Graphemes::from(""), 1), 17);
    }

    #[test]
    fn edit_distance_chinese_test() {
        assert_eq!(levenshtein_distance(&Graphemes::from("己所不欲勿施于人"), &Graphemes::from("back"), 1), 8);
        assert_eq!(levenshtein_distance(&Graphemes::from("back"), &Graphemes::from("己所不欲勿施于人"), 1), 8);
        assert_eq!(levenshtein_distance(&Graphemes::from("己所不欲勿施于人"), &Graphemes::from("不患人之不己知患不知人也"), 1), 10);
        assert_eq!(levenshtein_distance(&Graphemes::from("不患人之不己知患不知人也"), &Graphemes::from("己所不欲勿施于人"), 1), 10);
    }

    #[test]
    fn word_error_rate_test() {
        let mut dictionary : HashSet<Graphemes> = HashSet::new();
        dictionary.insert(Graphemes::from("we"));
        dictionary.insert(Graphemes::from("canon"));
        dictionary.insert(Graphemes::from("see"));
        dictionary.insert(Graphemes::from("ash"));
        dictionary.insert(Graphemes::from("ort"));
        dictionary.insert(Graphemes::from("distance"));
        dictionary.insert(Graphemes::from("ahead"));
        let predicted_sentence = max_match(&Graphemes::from("wecanonlyseeashortdistanceahead"), &dictionary);
        let actual_sentence = Graphemes::from("we can only see a short distance ahead");
        assert_eq!(word_error_rate(&actual_sentence, &predicted_sentence),0.625);
        assert_eq!(word_error_rate(&actual_sentence, &actual_sentence),0.0)
    }
}