use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut check = false;
    let mut container = HashSet::new();
    for value in nums {
        if container.contains(&value) {
            check = true;
            break;
        } else {
            container.insert(value);
        }
    }
    check
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duplicate() {
        assert!(contains_duplicate(vec![1, 2, 3, 1]));
        assert!(!contains_duplicate(vec![1, 2, 3, 4]));
        assert!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));
    }
}
