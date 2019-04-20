pub use self::graphemes_struct::Graphemes;

/// Vector of graphemes
mod graphemes_struct {
    extern crate unicode_segmentation;
    use unicode_segmentation::UnicodeSegmentation;
    use std::ops::{Deref, Index, IndexMut};
    use std::fmt::{Display, Formatter};
    use std::fmt;
    use len_trait::len::{Len, Empty, Clear};
    use push_trait::base::{Push, CanPush};

    /// Vector of graphemes
    #[derive(Debug, Hash, Eq, PartialEq)]
    pub struct Graphemes<'a> {
        graphemes : Vec<&'a str>,
    }

    impl<'a> Graphemes<'a> {
        pub fn new(string : &'a str) -> Graphemes<'a> {
            let graphemes = UnicodeSegmentation::graphemes(string, true).collect::<Vec<&str>>();
            Graphemes {
                graphemes,
            }
        }

        pub fn get(&self, index : usize) -> Option<&&str> {
            self.graphemes.get(index)
        }

        pub fn reverse(&mut self) {
            self.graphemes.reverse();
        }

        pub fn slice(&self, start : usize, end : usize) -> Self {
            let graphemes = self.graphemes[start..end].to_vec();
            Graphemes { graphemes }
        }

        pub fn append(&mut self, mut other : Graphemes<'a>) {
            self.graphemes.append(&mut other.graphemes);
        }

        pub fn split(&self, splitter : &'a str) -> Vec<Graphemes> {
            self.graphemes.split(|character| *character == splitter).map(
                |str_arr| Graphemes { graphemes: str_arr.to_vec()} ).collect()
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

    impl<'a> Empty for Graphemes<'a> {
        fn is_empty(&self) -> bool {
            self.graphemes.is_empty()
        }
    }

    impl<'a> Len for Graphemes<'a> {
        fn len(&self) -> usize {
            self.graphemes.len()
        }
    }

    impl<'a> Index<usize> for Graphemes<'a> {
        type Output = &'a str;

        fn index(&self, index: usize) -> & &'a str {
            &self.graphemes[index]
        }
    }

    impl<'a> IndexMut<usize> for Graphemes<'a> {
        fn index_mut(&mut self, index: usize) -> &mut &'a str  {
            &mut self.graphemes[index]
        }
    }

    impl<'a> Clear for Graphemes<'a> {
        fn clear(&mut self) {
            self.graphemes.clear();
        }
    }

    impl<'a> CanPush<&'a str> for Graphemes<'a> {
        type PushedOut = ();
    }

    impl<'a> Push<&'a str> for Graphemes<'a> {
        fn push(&mut self, val: &'a str) -> Option<Self::PushedOut> {
            self.graphemes.push(val);
            Some(())
        }
    }

    impl<'a> Default for Graphemes<'a> {
        fn default() -> Self {
            Graphemes::new("")
        }
    }


}

#[cfg(test)]
mod test_cases {
    use super::graphemes_struct::Graphemes;

    #[test]
    fn graphemes_split_test() {
        assert_eq!(Graphemes::new("hello world").split(" "), vec![Graphemes::new("hello"), Graphemes::new("world")])
    }
}