/*
    Anagram Check
    Given two strings, check if they are anagrams of each other. 
    Anagrams are words or phrases formed by rearranging the letters of another, 
    using all the original letters exactly once. 
    The strings may contain spaces or punctuation, but you need to ignore them while checking.

    You need to implement the function `are_anagrams(s1: String, s2: String) -> bool`.
    The function should return `true` if the two strings are anagrams, and `false` otherwise.

    Hint: Consider normalizing the strings by removing non-alphabetical characters and converting to lowercase before checking.
*/

use std::fmt::{self, Display, Formatter};

fn keep_alphabetical(s: String) -> String {
    let mut r = String::new();
    let temp_s = s.to_lowercase();
    for c in temp_s.chars() {
        if c >= 'a' && c <= 'z' {
            r.push(c);
        }
    }
    r
}

pub fn are_anagrams(s1: String, s2: String) -> bool {
    // TODO: Implement the logic to check if two strings are anagrams
    let mut s1 = keep_alphabetical(s1);
    let mut s2 = keep_alphabetical(s2);

    if s1.len() != s2.len() {
        return false;
    }
    let mut container = [0;26];
    for i in s1.chars() {
        let rd = container.get_mut(i as usize - 'a' as usize);
        match rd {
            Some(r) => {
                *r = *r + 1;
            },
            None => panic!("should be unreacheable"),
        }
    }
    
    for i in s2.chars() {
        let rd = container.get_mut(i as usize - 'a' as usize);
        match rd {
            Some(r) => {
                *r = *r - 1;
            },
            None => panic!("should be unreacheable"),
        }
    }

    for i in &container {
        if *i != 0 {
            return false;
        }
    }
    true// Placeholder return value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagram_1() {
        let s1 = "listen".to_string();
        let s2 = "silent".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_2() {
        let s1 = "evil".to_string();
        let s2 = "vile".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_3() {
        let s1 = "hello".to_string();
        let s2 = "world".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, false);
    }

    #[test]
    fn test_anagram_4() {
        let s1 = "Clint Eastwood".to_string();
        let s2 = "Old West Action".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_5() {
        let s1 = "Astronomer".to_string();
        let s2 = "Moon starer".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }
}
