use std::collections::HashMap;

pub fn solve(strs: Vec<&str>) -> Vec<Vec<&str>> {
    let mut map: HashMap<[u8; 26], Vec<&str>> = HashMap::new();

    for s in strs.into_iter() {
        let mut signature = [0u8; 26];
        for ch in s.chars() {
            let idx = (ch as u8 - b'a') as usize;
            signature[idx] += 1;
        }

        map.entry(signature).or_default().push(s);
    }

    map.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_returns_correctly_grouped_anagrams() {
        let mut actual = solve(vec!["eat", "tea", "tan", "ate", "nat", "bat"]);
        for group in &mut actual {
            group.sort();
        }
        actual.sort_by(|a, b| a[0].cmp(b[0]));

        let mut expected = vec![vec!["bat"], vec!["nat", "tan"], vec!["eat", "tea", "ate"]];
        for group in &mut expected {
            group.sort();
        }
        expected.sort_by(|a, b| a[0].cmp(b[0]));

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_empty_list_returns_single_group_with_empty_string() {
        assert_eq!(solve(vec![""]), vec![[""]]);
    }

    #[test]
    fn test_single_word_list_returns_single_group_with_word() {
        assert_eq!(solve(vec!["lol"]), vec![["lol"]]);
    }
}
