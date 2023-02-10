pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() < 3 {
        return vec![];
    }
    let mut nums = nums;
    nums.sort();
    vec![nums]
}

// #[cfg(test)]
// mod test {
//     use super::three_sum;
//
//     #[test]
//     fn test_three_sum() {
//         assert_eq!(three_sum(vec![0, 1, 1]), vec![vec![]]);
//         assert_eq!(
//             three_sum(vec![-1, 0, 1, 2, -1, -4]),
//             vec![vec![-1, -1, 2], vec![-1, 0, 1]]
//         );
//         assert_eq!(three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
//     }
// }
