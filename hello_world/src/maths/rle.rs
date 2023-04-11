
pub fn rle(str: &str) -> String {
    let mut result = String::new();
    
    let mut chars = str.chars();
    let Some(mut last) = chars.next() else { return String::new(); };
    let mut amount = 1;

    for c in chars {
        if c == last {
            amount += 1;
        } else {
            result += &format!("{last}{amount}");
            amount = 1;
        }
        last = c;
    }
    result += &format!("{last}{amount}");
    result.to_string()
}

#[test]
fn test_rle() {
    let s: &str = &String::new();
    assert_eq!(rle(""), "");
    assert_eq!(rle("a"), "a1");
    assert_eq!(rle("aa"), "a2");
    assert_eq!(rle("aaa"), "a3");
    assert_eq!(rle("ab"), "a1b1");
    assert_eq!(rle("aab"), "a2b1");
    assert_eq!(rle("aaab"), "a3b1");
    assert_eq!(rle("aaabb"), "a3b2");
    assert_eq!(rle("aaabbb"), "a3b3");
    assert_eq!(rle("aaabbbccc"), "a3b3c3");
    assert_eq!(rle("aaabbbcccd"), "a3b3c3d1");
    assert_eq!(rle("aaabbbcccdde"), "a3b3c3d2e1");
}



use std::{fs, path::Path};

// Given a file, return the longest line in the file
pub fn longest_line(content: &str) -> Option<&str> {
    content.lines().max_by_key(|line| line.len())
}

// Given a file, return the line with the most occurrences of a given character
pub fn lines_with_most(content: &str, c: char) -> Option<&str> {
    content
        .lines()
        .max_by_key(|line| line.chars().filter(|&x| x == c).count())
}

// Check if a file is bigger than a given size in bytes
pub fn is_bigger_than(path: impl AsRef<Path>, size: u64) -> Result<bool, std::io::Error> {
    // This function currently corresponds to the stat function on Unix and the GetFileInformationByHandle function on Windows. Note that, this may change in the future.
    let metadata = fs::metadata(path)?;
    Ok(metadata.len() > size)
}



pub fn run_length_encoding(s: &str) -> String {
    s.chars()
        .fold((String::new(), 0, ' '), |(mut acc, mut count, mut prev), c| {
            if c == prev {
                count += 1;
            } else {
                if count > 0 {
                    acc.push_str(&format!("{}{}", count, prev));
                }
                count = 1;
                prev = c;
            }
            (acc, count, prev)
        })
        .0
}
