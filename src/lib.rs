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

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::error::Error;

pub const LOREMIPSUM: &str = include_str!("loremipsum.txt");

/// `lorem_para` returns multiples of paragraphs.
///
/// # Examples
///
/// ```
/// use loremipsum::{paragraph, lorem_para};
///
/// let para = paragraph();
/// let paragraph_1 = lorem_para(1usize);
/// assert_eq!(para.len(), paragraph_1.len());
/// let paragraph_2 = lorem_para(2usize);
/// assert_eq!(para.len() * 2usize,  paragraph_2.len());
/// ```
pub fn lorem_para(count: usize) -> String {
    let mut lorem: String = String::new();
    let paragraph: String = paragraph();
    (0..count).for_each(|_: usize| {
        lorem.push_str(&paragraph);
    });

    lorem
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Lorem {
    pub paragraph: String,
}

impl Default for Lorem {
    fn default() -> Self {
        Self::new()
    }
}
impl Lorem {
    pub fn chars(count: usize) -> String {
        let mut lorem: String = String::new();
        let new: Lorem = Self::new();
        (0..count).for_each(|_: usize| {
            lorem.push(
                new.paragraph
                    .chars()
                    .next()
                    .expect("loremipsum::Lorem::chars() failed to get next char."),
            );
        });
        lorem
    }

    pub fn len_letters() -> usize {
        Self::new().paragraph.chars().count()
    }
    pub fn len_paras() -> usize {
        Self::new().paragraph.split_terminator('.').count()
    }
    pub fn len_words() -> usize {
        Self::new().paragraph.split_whitespace().count()
    }

    fn new() -> Self {
        Self {
            paragraph: paragraph(),
        }
    }

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
    /// let len_words: usize = Lorem::len_words();
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
        let mut lorem: String = String::new();
        let new: Lorem = Self::new();
        (0..count).for_each(|_: usize| {
            lorem.push_str(
                new.paragraph
                    .split_whitespace()
                    .next()
                    .expect("loremipsum::Lorem::words() failed to get next word."),
            );
            lorem.push(' ');
        });
        lorem
    }
}

/// `paragraph` returns a single paragraphs of lorem ipsum text.
///
/// # Examples
///
/// ```
/// use loremipsum::paragraph;
///
/// let para = paragraph();
/// let words = para.split_whitespace();
/// assert_eq!(words.last().unwrap(), "laborum.");
/// let para: String = paragraph();
/// let words: Vec<&str> = para.split_whitespace().collect::<Vec<_>>();
/// assert_eq!(words.first().unwrap().to_string(), String::from("Lorem"));
/// ```
///
/// - Word and letter counts:
///
/// ```
/// use loremipsum::paragraph;
///
/// let para = paragraph();
/// assert_eq!(para.len(), 440usize);
/// let words = para.split_whitespace();
/// assert_eq!(words.count(), 64usize);
/// ```
pub fn paragraph() -> String {
    let new_paragraph: Vec<String> = run().expect("Failed to get lorem ipsum words");
    let mut para_buf_as_string = String::new();
    new_paragraph.iter().for_each(|word: &String| {
        para_buf_as_string.push_str(word);
        para_buf_as_string.push(' ');
    });

    para_buf_as_string.trim().to_string()
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

    #[test]
    fn it_read_lorem() {
        let string = read_lorem().expect("Failed to read lorem ipsum from file");
        assert_eq!(string.len(), 440usize);
        let words = string.split_ascii_whitespace();
        assert_eq!(words.count(), 64usize);
    }

    #[test]
    fn it_gets_array_words() {
        let got = run();
        assert!(got.is_ok(), "Failed to get array of words from lorem ipsum");

        let len = got.unwrap().len();
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
    fn it_lorem_para() {
        let para = paragraph();
        let paragraph_1 = lorem_para(1usize);
        assert_eq!(para.len(), paragraph_1.len());
        let paragraph_2 = lorem_para(2usize);
        assert_eq!(para.len() * 2usize, paragraph_2.len());
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
    fn it_lorem_chars() {
        let count = 5usize;
        let chars: String = Lorem::chars(count);
        assert_eq!(chars.len(), count);
    }
    #[test]
    fn it_lorem_words() {
        let len_words: usize = Lorem::len_words();

        (1..=len_words).for_each(|i: usize| {
            let words_count: usize = Lorem::words(i).split_whitespace().count();
            assert_eq!(words_count, i);

            let len_words_rest: usize = Lorem::words(len_words - i).split_whitespace().count();
            assert_eq!(words_count, len_words - len_words_rest);
        });
    }
    #[test]
    fn it_lorem_paragraphs() {
        let count = 5usize;
        let paragraphs: String = Lorem::paragraphs(count);
        let len: usize = Lorem::len_paras();
        assert_eq!(paragraphs.split_terminator('.').count(), len * count);
    }
}

/* // Not public API. Referenced by macro-generated code.
// (Placeholder: credits to anyhow lib)
#[doc(hidden)]
pub mod __private {
    use anyhow::Error;
    use core::fmt::Arguments;

    #[doc(hidden)]
    #[inline]
    #[cold]
    pub fn format_err(args: Arguments) -> Error {
        #[cfg(anyhow_no_fmt_arguments_as_str)]
        let fmt_arguments_as_str = None::<&str>;
        #[cfg(not(anyhow_no_fmt_arguments_as_str))]
        let fmt_arguments_as_str = args.as_str();

        if let Some(message) = fmt_arguments_as_str {
            // anyhow!("literal"), can downcast to &'static str
            Error::msg(message)
        } else {
            // anyhow!("interpolate {var}"), can downcast to String
            Error::msg(format!("{}", args))
        }
    }
}
 */
