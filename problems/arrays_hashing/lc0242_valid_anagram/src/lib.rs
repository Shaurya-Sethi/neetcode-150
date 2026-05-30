use std::collections::HashMap;
pub fn solve(s: String, t: String) -> bool {
    if !s.len() == t.len() {
        return false;
    }
    let mut map = HashMap::new();
    for b in s.as_bytes() {
        map.entry(b).and_modify(|count| *count += 1).or_insert(1);
    }
    for b in t.as_bytes() {
        map.entry(b).and_modify(|count| *count -= 1).or_insert(1);
    }
    map.into_iter().all(|(_, count)| count == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_returns_true() {
        let s: String = "anagram".to_string();
        let t: String = "nagaram".to_string();
        assert!(solve(s, t));
    }

    #[test]
    fn test_not_anagram_returns_false() {
        let s: String = "batman".to_string();
        let t: String = "tabnan".to_string();
        assert!(!solve(s, t));
    }
}
