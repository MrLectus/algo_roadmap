use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut freq = HashMap::new();
    for (index, value) in nums.iter().enumerate() {
        let difference = target - value;
        if freq.contains_key(&difference) {
            return vec![*freq.get(&difference).unwrap(), index as i32];
        } else {
            freq.insert(value, index as i32);
        }
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
