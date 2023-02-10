use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut count = HashMap::new();
    for value in &nums {
        *count.entry(value).or_insert(0) += 1;
    }
    let len = nums.len() + 1;
    let mut freq: Vec<Vec<i32>> = vec![vec![]; len];
    count.iter().for_each(|(key, value)| {
        freq[*value].push(**key);
    });
    let mut ans = Vec::new();
    for x in (0..freq.len()).rev() {
        for y in &freq[x] {
            ans.push(*y);
        }
        if ans.len() == k as usize {
            break;
        }
    }
    ans
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_k_frequent() {
        let mut test1 = top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
        test1.sort();
        let mut test2 = top_k_frequent(vec![1], 1);
        test2.sort();
        let mut test3 = top_k_frequent(vec![3, 3, 3, 2, 2], 2);
        test3.sort();
        let mut test4 = top_k_frequent(vec![1, 1, 1, 1], 1);
        test4.sort();
        assert_eq!(test1, vec![1, 2]);
        assert_eq!(test2, vec![1]);
        assert_eq!(test3, vec![2, 3]);
        assert_eq!(test4, vec![1]);
    }
}
