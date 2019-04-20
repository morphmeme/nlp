pub use self::graphemes_struct::Graphemes;

/// Vector of graphemes
mod graphemes_struct {
    extern crate unicode_segmentation;
    use unicode_segmentation::UnicodeSegmentation;
    use std::ops::Deref;
    use std::fmt::{Display, Formatter};
    use std::fmt;

    /// Vector of graphemes
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

        pub fn push(&mut self, grapheme : &'a str) {
            self.graphemes.push(grapheme);
        }

        pub fn get(&self, index : usize) -> Option<&&str> {
            self.graphemes.get(index)
        }

        pub fn at(&self, index : usize) -> &str {
            self.graphemes[index]
        }

        pub fn reverse(&mut self) {
            self.graphemes.reverse();
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
}
