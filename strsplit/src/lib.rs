//#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
#![warn(rust_2018_idioms)]

#[derive(Debug)]
pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    delimiter: D,
}

// str -> [char]
// &str -> &[char]
// String -> Vec<char>

impl<'haystack, D> StrSplit<'haystack, D> {
    ///
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl<'haystack, D> Iterator for StrSplit<'haystack, D>
where
    D: Delimiter,
{
    type Item = &'haystack str;

    fn next(&mut self) -> Option<Self::Item> {
        // here the following two are equivalent:
        // if let Some(ref mut remainder) = self.remainder {
        // if let Some(remainder) = &mut self.remainder {

        if let Some(remainder) = self.remainder.as_mut() {
            if let Some((delim_start, delim_end)) = self.delimiter.find_next(remainder) {
                let until_delimiter = &remainder[..delim_start];
                *remainder = &remainder[delim_end..];
                Some(until_delimiter)
            } else {
                self.remainder.take()
            }
        } else {
            None
        }
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.chars()
            .position(|c| c == *self)
            .map(|start| (start, start + self.len_utf8()))
    }
}

pub fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, c).next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn until_char_test() {
        assert_eq!(until_char("hello world", 'o'), "hell");
    }

    #[test]
    fn it_works() {
        let haystack = "a b c d e";
        let letters = StrSplit::new(haystack, " ").collect::<Vec<_>>();
        assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn tail() {
        let haystack = "a b c d ";
        let letters = StrSplit::new(haystack, " ").collect::<Vec<_>>();
        assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
    }
}
