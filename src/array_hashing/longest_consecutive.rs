use std::collections::HashSet;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let freq = nums.iter().copied().collect::<HashSet<i32>>();
    let mut max_len = 0;
    for i in nums.iter() {
        if !freq.contains(&(i - 1)) {
            let mut length = 0;
            while freq.contains(&(i + length)) {
                length += 1;
            }
            max_len = max_len.max(length);
        }
    }
    max_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_consecutive_sequence() {
        assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
        assert_eq!(
            longest_consecutive(vec![0, 0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
            9
        );
        assert_eq!(
            longest_consecutive(vec![-1, -2, -3, -4, -5, 100, 101, 102]),
            5
        );
    }
}
