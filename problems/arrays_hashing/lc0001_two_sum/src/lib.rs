use std::collections::HashMap;

pub fn solve(list: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::with_capacity(list.len());
    for (ind, num) in list.into_iter().enumerate() {
        let complement = target - num;
        if let Some(&index) = map.get(&complement) {
            return vec![index as i32, ind as i32];
        }
        map.insert(num, ind);
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_returns_correct_indexes() {
        let mut actual = solve(vec![2, 7, 11, 15], 9);
        actual.sort_unstable();
        let mut expected = vec![0, 1];
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_duplicates_return_correct_indexes() {
        let mut actual = solve(vec![3, 3], 6);
        actual.sort_unstable();
        let mut expected = vec![0, 1];
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_negative_numbers() {
        let mut actual = solve(vec![-3, 4, 3, 90], 0);
        actual.sort_unstable();
        let mut expected = vec![0, 2];
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }
}
