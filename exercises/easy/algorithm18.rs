/*
    Merge Intervals
    Given an array of intervals where each interval is represented by a pair of integers [start, end], 
    merge all overlapping intervals and return a list of non-overlapping intervals.
    
    The intervals are inclusive, meaning the interval [start, end] includes both start and end points.
    
    You need to implement the function `merge_intervals(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>>`.
    The function should return a vector containing all the merged intervals.

    Hint: You can start by sorting the intervals by their starting point and then merge them one by one.
*/

use std::fmt::{self, Display, Formatter};

pub fn merge_intervals(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    //  Implement the logic to merge overlapping intervals
    let mut intervals = intervals;
    intervals.sort_by(|a, b| {
        a.get(0).unwrap().cmp(b.get(0).unwrap())
    });
    
    let mut i = 0;
    while i < intervals.len() - 1 {
        let pre = intervals.get(i).unwrap();
        let post = intervals.get(i + 1).unwrap();
        
        {
            let post_start = *post.get(0).unwrap();
            let post_end = *post.get(1).unwrap();
            if post_start >= *pre.get(0).unwrap() && post_start <= *pre.get(1).unwrap() {
                // 有重叠
                if post_end >= *pre.get(0).unwrap() && post_end <= *pre.get(1).unwrap() {
                    // 将i + 1抛出多余
                    intervals.remove(i + 1);
                }else {
                    // 更新pre_end为post_end
                    *intervals.get_mut(i).unwrap().get_mut(1).unwrap() = post_end;
                    intervals.remove(i + 1);
                
                }
            }else {
                i += 1;
            }
        }
    }
    intervals
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_intervals_1() {
        let intervals = vec![
            vec![1, 3],
            vec![2, 6],
            vec![8, 10],
            vec![15, 18]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![1, 6],
            vec![8, 10],
            vec![15, 18]
        ]);
    }

    #[test]
    fn test_merge_intervals_2() {
        let intervals = vec![
            vec![1, 4],
            vec![4, 5]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![1, 5]
        ]);
    }

    #[test]
    fn test_merge_intervals_3() {
        let intervals = vec![
            vec![1, 4],
            vec![0, 4]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![0, 4]
        ]);
    }

    #[test]
    fn test_merge_intervals_4() {
        let intervals = vec![
            vec![1, 10],
            vec![2, 6],
            vec![8, 10]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![1, 10]
        ]);
    }

    #[test]
    fn test_merge_intervals_5() {
        let intervals = vec![
            vec![1, 2],
            vec![3, 5],
            vec![4, 7],
            vec![8, 10]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![1, 2],
            vec![3, 7],
            vec![8, 10]
        ]);
    }
}
