/*
    Longest Substring Without Repeating Characters
    Given a string, find the length of the longest substring without repeating characters. 
    The substring must not contain any duplicate characters, and its length should be maximized.

    You need to implement the function `longest_substring_without_repeating_chars(s: String) -> i32`.
    The function should return the length of the longest substring without repeating characters.
    
    Hint: Consider using the sliding window technique to efficiently solve this problem in O(n) time complexity.
*/

use std::fmt::{self, Display, Formatter};


fn have_repeating_c(c: &[i32]) -> bool {
    for i in c {
        if *i > 1 {
            return true;
        }
    }
    false
}

pub fn longest_substring_without_repeating_chars(s: String) -> i32 {
    // : Implement the logic to find the longest substring without repeating characters
    let mut r = 0; // Placeholder return value
    let mut i = 0;
    let mut j = 0;
    let bytes = s.as_bytes();
    let len = bytes.len();
    let mut container = [0;26];
    while j < len {
        let c = *bytes.get(j).unwrap() as i32 - 'a' as i32;
        let c1 = container.get_mut(c as usize);
        match c1 {
            Some(aa) => {*aa = *aa + 1},
            None => panic!("should be unreachable"),
        }
        while i < j && have_repeating_c(&container) {
            let c = *bytes.get(i).unwrap() as i32 - 'a' as i32;

            let c1 = container.get_mut(c as usize);
            match c1 {
                Some(aa) => {*aa = *aa - 1},
                None => panic!("should be unreachable"),
            }
            i += 1;
        }
        r = r.max(j - i + 1);
        j += 1;
    }
    r as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_substring_1() {
        let s = "abcabcbb".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 3);  // "abc"
    }

    #[test]
    fn test_longest_substring_2() {
        let s = "bbbbb".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 1);  // "b"
    }

    #[test]
    fn test_longest_substring_3() {
        let s = "pwwkew".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 3);  // "wke"
    }

    #[test]
    fn test_longest_substring_4() {
        let s = "".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 0);  // Empty string
    }

    #[test]
    fn test_longest_substring_5() {
        let s = "abcde".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 5);  // "abcde"
    }
}
