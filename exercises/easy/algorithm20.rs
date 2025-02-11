/*
    Sum of Two Integers
    Given two integers, calculate their sum without using the `+` operator. 
    You need to implement the function `get_sum(a: i32, b: i32) -> i32`.
    The function should return the sum of the two integers `a` and `b`.

    Hint: You can solve this problem using bitwise operations.
*/

use std::fmt::{self, Display, Formatter};

pub fn get_sum(a: i32, b: i32) -> i32 {
    //  Implement the logic to calculate the sum of two integers without using `+`
    let len = i32::BITS as i32;
    let mut sum = 0;
    let mut carry = false;
    let mut i = 0;
    while i  < len {
        let left = (a as u32 & (1 << i)) >> i;
        let right = (b as u32 & (1 << i)) >> i;
        let mut cur  = 0;
        if left == 1 && right == 1 {
            if carry {
                cur = 1;
            }else {
                carry = true;
                cur = 0;
            }
        }else if left == 1 || right == 1 {
            if carry {
                cur = 0;
            }else {
                cur = 1;
            }
        }else {
            if carry {
                cur = 1;
                carry = false;
            }else {
                cur = 0;
            }
        }

        if cur == 1 {
            sum =  sum | (cur << i);
        }

        i += 1;
    }
    

    sum // Placeholder return value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_1() {
        let result = get_sum(1, 2);
        println!("Sum of 1 and 2: {}", result);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_sum_2() {
        let result = get_sum(-1, 1);
        println!("Sum of -1 and 1: {}", result);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_sum_3() {
        let result = get_sum(100, 200);
        println!("Sum of 100 and 200: {}", result);
        assert_eq!(result, 300);
    }

    #[test]
    fn test_sum_4() {
        let result = get_sum(-50, -50);
        println!("Sum of -50 and -50: {}", result);
        assert_eq!(result, -100);
    }

    #[test]
    fn test_sum_5() {
        let result = get_sum(0, 0);
        println!("Sum of 0 and 0: {}", result);
        assert_eq!(result, 0);
    }
}
