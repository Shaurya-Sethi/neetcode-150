pub fn solve(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut arr = [0; 26];
    for (sb, tb) in s.as_bytes().iter().zip(t.as_bytes().iter()) {
        arr[(sb - b'a') as usize] += 1;
        arr[(tb - b'a') as usize] -= 1;
    }
    arr.into_iter().all(|count| count == 0)
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
