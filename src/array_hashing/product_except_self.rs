pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let prefix = nums
        .iter()
        .scan(1, |x, y| {
            *x *= y;
            Some(*x)
        })
        .collect::<Vec<i32>>();
    let mut suffix = nums
        .iter()
        .rev()
        .scan(1, |x, y| {
            *x *= y;
            Some(*x)
        })
        .collect::<Vec<i32>>();
    suffix.reverse();
    let mut y = 2;
    let mut products = Vec::new();
    products.push(suffix[1]);
    for x in 0..nums.len() - 1 {
        let xprod = prefix.get(x).unwrap_or(&1);
        let yprod = suffix.get(y).unwrap_or(&1);
        products.push(xprod * yprod);
        y += 1;
    }
    products
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_except_self() {
        assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
        assert_eq!(
            product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }
}
