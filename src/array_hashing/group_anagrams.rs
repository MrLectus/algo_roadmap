use std::collections::{BTreeMap, HashMap};
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut freq = HashMap::new();
    for words in strs {
        let mut key = BTreeMap::new();
        for x in words.chars() {
            *key.entry(x).or_insert(0) += 1;
        }
        freq.entry(key).or_insert_with(Vec::new).push(words);
    }
    freq.values().cloned().collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_anagram() {
        let mut test_v1 = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];

        test_v1.sort();

        let mut test_v2 = vec![
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
        ];
        test_v2.iter_mut().for_each(|v2| v2.sort());

        let mut test_v1 = group_anagrams(test_v1);
        test_v1.sort();
        test_v2.sort();
        assert_eq!(test_v1, test_v2);
    }
}
