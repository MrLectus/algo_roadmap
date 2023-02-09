use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut freq1 = HashMap::new();
    let mut freq2 = HashMap::new();
    for (value1, value2) in s.chars().zip(t.chars()) {
        *freq1.entry(value1).or_insert(0) += 1;
        *freq2.entry(value2).or_insert(0) += 1;
    }
    freq1 == freq2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagram() {
        assert!(is_anagram("anagram".to_string(), "nagaram".to_string()));
        assert!(!is_anagram("rat".to_string(), "car".to_string()));
    }
}
