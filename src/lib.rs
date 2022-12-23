//! [![github]](https://github.com/lloydlobo/loremipsum)&ensp;[![crates-io]](https://crates.io/crates/loremipsum)&ensp;[![docs-rs]](https://docs.rs/loremipsum)
//!
// TODO: [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
// TODO: [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
// TODO: [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! <br>
//!
//! This library provides [`loremipsum::run`][run], a lorem ipsum word generator.
//!
//! <br>
//!
//! # Details (Placeholder: credits to anyhow lib)
//!
//! - Use `loremipsum::run(words)` as the generator to get the desired `String` of
//!   lorem ipusm of count `words`.
//!
//!   ```
//!   # pub trait Deserialize {}
//!   #
//!   # mod serde_json {
//!   #     use super::Deserialize;
//!   #     use std::io;
//!   #
//!   #     pub fn from_str<T: Deserialize>(json: &str) -> io::Result<T> {
//!   #         unimplemented!()
//!   #     }
//!   # }
//!   #
//!   # struct ClusterMap;
//!   #
//!   # impl Deserialize for ClusterMap {}
//!   #
//!   use loremipsum::*;
//!
// TODO: Implement word count feature.
//!   fn get_loremipsum() -> String {
//!       let word_count = 5;
//!       let lorem = loremipsum::run();
//!       // assert_eq!(lorem, String::from("Lorem ipsum dolor sit amet");
//!       // assert_eq!(lorem.split_ascii_whitespace().len(), word_count);
//!       lorem.unwrap().join(" ")
//!   }
//!   #
//!   # fn main() {}
//!   ```
//! See https://docs.rs/lipsum/0.8.2/src/lipsum/lib.rs.html#427-429.
//!
//! #Dev
//!
//! # ```bash
//! # cargo watch -x "test -p loremipsum"
//! # ```

#![forbid(unsafe_code)]
#![deny(missing_docs)]

use anyhow::Result;
use std::{error::Error, str::FromStr};

/// The generic lorem ipsum text read from file.
///
/// See [wiki]: https://en.wikipedia.org/wiki/Lorem_ipsum
pub const LOREMIPSUM: &str = include_str!("loremipsum.txt");

/// Implements struct for common methods to get lorem ipsum texts.
///
/// # Example
///
/// ```
/// use loremipsum::Lorem;
/// let text: String = Lorem::new().paragraph;
/// assert_eq!(text.split_terminator('.').count(), Lorem::count_paras());
/// assert_eq!(text.split_whitespace().count(), Lorem::count_words());
/// assert_eq!(text.chars().count(), Lorem::count_chars());
/// ```
#[derive(Clone, Debug)]
pub struct Lorem {
    /// The default lorem impsum text.
    pub paragraph: String,
}

/// Implements `Default` for [`Lorem`].
impl Default for Lorem {
    fn default() -> Self {
        Self::new()
    }
}

/// Implements `Lorem` struct.
impl Lorem {
    /// Returns `count` number of characters.
    ///
    /// # Example
    ///
    /// ```
    /// use loremipsum::Lorem;
    /// let chars: String = Lorem::chars(5usize);
    /// assert_eq!(chars, String::from("Lorem"));
    /// ```
    ///
    /// It's important to remember that char represents a Unicode Scalar Value, and might not match your idea of what a 'character' is. Iteration over grapheme clusters may be what you actually want. This functionality is not provided by Rust's standard library, check crates.io instead.  
    ///
    ///
    pub fn chars(count: usize) -> String {
        let count_max: usize = Lorem::count_chars();
        match count <= count_max {
            true => Self::new().paragraph.chars().take(count).collect(),
            false => {
                let paragraph: String = Self::new().paragraph;
                let mut join_new = String::new();
                (0..(count % count_max)).for_each(|_: usize| {
                    (0..count_max).for_each(|i: usize| {
                        // 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9....
                        join_new.push_str(&paragraph.chars().nth(i).unwrap().to_string());
                    });
                });
                paragraph + join_new.as_str()
            }
        }
    }

    /// Returns count of total `chars` in a [`Lorem`] instance.
    /// - default is `440usize`.
    pub fn count_chars() -> usize {
        Self::new().paragraph.chars().count()
    }

    /// Returns count of total paragraph in a [`Lorem`] instance.
    /// - default is `1usize`.
    pub fn count_paras() -> usize {
        Self::new().paragraph.split_terminator('.').count()
    }

    /// Returns count of total words in a [`Lorem`] instance.
    /// - default is `64usize`.
    pub fn count_words() -> usize {
        Self::new().paragraph.split_whitespace().count()
    }

    /// `create_new` returns a single paragraphs of lorem ipsum text.
    ///
    /// It is used in the `new()` method for [`Lorem`].
    ///
    /// # Examples
    ///
    /// ```
    /// use loremipsum::Lorem;
    ///
    /// let para = Lorem::new().paragraph;
    /// let words = para.split_whitespace();
    /// assert_eq!(words.last().unwrap(), "laborum.");
    ///
    /// let para = Lorem::new().paragraph;
    /// let words: Vec<&str> = para.split_whitespace().collect::<Vec<_>>();
    /// assert_eq!(words.first().unwrap().to_string(), String::from("Lorem"));
    ///
    /// ```
    ///
    /// - Word and letter counts:
    ///
    /// ```
    /// use loremipsum::Lorem;
    ///
    /// let para = Lorem::new().paragraph;
    /// assert_eq!(para.len(), 440usize);
    ///
    /// let words = para.split_whitespace();
    /// assert_eq!(words.count(), 64usize);
    ///
    /// ```
    fn create_new() -> String {
        let new_paragraph: Vec<String> = run().expect("Failed to get lorem ipsum words");

        let mut para_buf_as_string = String::new();

        new_paragraph.iter().for_each(|word: &String| {
            para_buf_as_string.push_str(word);
            para_buf_as_string.push(' ');
        });

        para_buf_as_string.trim().to_string()
    }

    /// Returns the `is_empty` of this [`Lorem`].
    ///
    /// # Example
    ///
    /// ```
    /// use loremipsum::Lorem;
    /// let self_new = Lorem::new();
    /// assert!(!Lorem::is_empty(&self_new));
    /// ```
    pub fn is_empty(&self) -> bool {
        self.paragraph.len() == 0
    }

    /// Creates a new [`Lorem`].
    pub fn new() -> Self {
        Self {
            paragraph: Self::create_new(),
        }
    }

    /// Creates a new [`Lorem`] instance of paragraphs.
    pub fn paragraphs(count: usize) -> String {
        let mut lorem: String = String::new();
        let new: Lorem = Self::new();
        (0..count).for_each(|_: usize| {
            lorem.push_str(&new.paragraph);
        });
        lorem
    }

    /// Implements `Lorem::words(count: usize)` to generate count number of words.
    ///
    /// # Example
    ///
    ///  ```
    /// use loremipsum::Lorem;
    /// let len_words: usize = Lorem::count_words();
    /// (1..=len_words).for_each(|i: usize| {
    ///     let words_count: usize = Lorem::words(i).split_whitespace().count();
    ///     assert_eq!(words_count, i);
    ///     let len_words_rest: usize = Lorem::words(len_words - i).split_whitespace().count();
    ///     assert_eq!(words_count, len_words - len_words_rest);
    /// });
    /// ```
    ///
    /// # Panics
    ///
    /// - If count is greater than `Lorem::len_words()`
    ///
    /// TODO: Make len_words()... to constants.
    /// #```rust
    /// #use Lorem::{words, len_words};
    /// #assert_eq!(words(len_words()*2),len_words()*2)
    /// #```
    /// ```bash
    /// ---- tests::it_lorem_words stdout ----
    /// thread 'tests::it_lorem_words' panicked at 'called `Option::unwrap()` on a `None` value', loremipsum/src/lib.rs:133:41
    /// ````
    pub fn words(count: usize) -> String {
        let mut result: Vec<&str> = Vec::<&str>::new();
        let count_default: usize = Lorem::count_words();

        let new: Lorem = Lorem::new();
        let source: Vec<&str> = new.paragraph.split_whitespace().collect::<Vec<&str>>();

        (1..=count).for_each(|i: usize| {
            result.push(source[((i - 1) % count_default)]);
            result.push(" ");
        });

        String::from(result.concat().trim())
    }
}

/// Implements `run` that returns a `Result` containing a `Vec<String>` of words
/// from the lorem ipsum text file.
///
/// # Examples
///
/// ```rust
/// use loremipsum::*;
///
/// let got: Vec<String> = run().unwrap();
/// let got_first: &String = &got[0];
/// let expect_first = "Lorem";
/// assert_eq!(got_first, expect_first);
///
/// ```
/// ```
/// # // Archive
/// #
/// # use loremipsum::*;
/// #
/// # let lorem_ipsum = read_lorem().expect("Failed to read lorem ipsum from file");
/// # let mut result = Vec::new();
/// # (1..=lorem_ipsum.len()).for_each(|i: usize| {
/// #     result = lorem_ipsum.split::<char>(' ').take(i).map(String::from).collect::<Vec<String>>();
/// # });
/// ````
pub fn run() -> Result<Vec<String>, Box<dyn Error>> {
    let lorem_ipsum: String = read_lorem()?;
    let words: Vec<String> = lorem_ipsum
        .split_whitespace()
        .map(|word| word.to_string())
        .collect();

    Ok(words)
}

/// Implements `read_lorem` that returns a `Result` containing a `String
/// of the 'loremipsum` text data file.
///
/// # Examples
///
/// ```rust
/// use loremipsum::*;
///
/// let string = read_lorem().expect("Failed to read lorem ipsum from file");
/// assert_eq!(string.len(), 440usize);
/// let words = string.split_ascii_whitespace();
/// assert_eq!(words.count(), 64usize);
/// ```
pub fn read_lorem() -> Result<String, Box<dyn Error>> {
    Ok(LOREMIPSUM
        .lines()
        .map(|line| line.trim())
        .into_iter()
        .collect::<String>())
}

//////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;

    // #[test]
    fn it_lorem_chars() {
        let len: usize = Lorem::count_chars();
        let chars_count = Lorem::chars(len).chars().count();
        assert_eq!(chars_count, len);
        // TODO: Delete the mock len of 5usize below.
        let len = len * 2;
        (1..=len).for_each(|i: usize| {
            let chars_count = Lorem::chars(i).chars().count();
            assert_eq!(chars_count, i);
        });
        let chars_count = Lorem::chars(886).chars().count();
        assert_eq!(chars_count, 886);
    }

    #[test]
    fn it_lorem_words() {
        let len: usize = Lorem::count_words() * 2usize;
        (1..=len).for_each(|i: usize| {
            let words_count: usize = Lorem::words(i).split_whitespace().count();
            assert_eq!(words_count, i);
            let len_words_rest: usize = Lorem::words(len - i).split_whitespace().count();
            assert_eq!(words_count, len - len_words_rest);
        });
    }

    #[test]
    fn it_lorem_paragraphs() {
        let len: usize = Lorem::count_paras();
        (1..=len).for_each(|i: usize| {
            let paras: String = Lorem::paragraphs(i);
            assert_eq!(paras.split_terminator('.').count(), len * i);
        });
    }

    #[test]
    fn it_new_lorem() {
        let para: String = Lorem::new().paragraph;
        let words: str::SplitWhitespace = para.split_whitespace();
        assert_eq!(
            "laborum.",
            words
                .clone()
                .last()
                .expect("Failed to get last word from lorem ipsum paragraph"),
        );
        assert_eq!(
            "Lorem",
            *words
                .collect::<Vec<&str>>()
                .first()
                .expect("Failed to get first word from lorem ipsum paragraph")
        );
    }

    #[test]
    fn it_read_lorem() {
        let string: String = read_lorem().expect("Failed to read lorem ipsum from file");
        assert_eq!(string.len(), 440usize);
        let words: str::SplitAsciiWhitespace = string.split_ascii_whitespace();
        assert_eq!(words.count(), 64usize);
    }

    #[test]
    fn it_gets_array_words() {
        let got: Result<Vec<String>, Box<dyn Error>> = run();
        assert!(got.is_ok(), "Failed to get array of words from lorem ipsum");

        let len: usize = got.unwrap().len();
        assert_eq!(len, 64usize);
    }

    #[test]
    fn it_matches_run_array_first_last_word() {
        let got: Vec<String> = run().unwrap();

        let got_first: &String = &got[0];
        let expect_first = "Lorem";
        assert_eq!(got_first, expect_first);

        let last: &String = &got[got.len() - 1usize];
        let expect_last = "laborum.";
        assert_eq!(last, expect_last);
    }

    #[test]
    fn it_matches_run_array_all_words() {
        let binding: String = read_lorem().expect("Failed to read lorem ipsum from file");
        let source: Vec<&str> = binding.split::<char>(' ').collect();

        let derive: Vec<String> = run().expect("Failed to get array of words from run_array");
        assert_eq!(source.len(), derive.len());

        (1..derive.len()).for_each(|i: usize| {
            let x: &str = source[i];
            let y: &String = &derive[i];
            assert_eq!(x, *y);
            assert_eq!(x.len(), y.len());
        });
    }

    #[test]
    fn it_wraps_words() {
        let count_words: usize = Lorem::count_words();
        (0..count_words * 4usize).for_each(|i: usize| {
            let words = Lorem::words(count_words * i);
            assert_eq!(words.split_whitespace().count(), count_words * i);
        });
    }
}
