pub fn max_area(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut max = 0;
    while left < right {
        if height[left] < height[right] {
            max = max.max(height[left] * (right - left) as i32);
            left += 1;
        } else {
            max = max.max((right - left) as i32 * height[right]);
            right -= 1;
        }
    }
    max
}

#[cfg(test)]
mod test {
    use crate::two_pointers::max_area::max_area;

    #[test]
    fn test_max_area() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(max_area(vec![1, 1]), 1);
    }
}
