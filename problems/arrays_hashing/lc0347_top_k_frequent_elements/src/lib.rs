use std::collections::HashMap;
pub fn solve(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for num in nums.into_iter() {
        map.entry(num).and_modify(|count| *count += 1).or_insert(1);
    }
    let mut counts: Vec<(i32, i32)> = map.into_iter().collect();
    counts.sort_by_key(|v| v.1);
    let mut k = k;
    let mut answer = Vec::with_capacity(k as usize);
    while k > 0 {
        if let Some((num, _count)) = counts.pop() {
            answer.push(num);
        }
        k -= 1;
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_returns_correct_values() {
        let mut actual = solve(vec![1, 1, 1, 2, 2, 3], 2);
        actual.sort_unstable();
        let mut expected = vec![1, 2];
        expected.sort_unstable();
        assert_eq!(actual, expected);

        let mut actual = solve(vec![1, 2, 1, 2, 1, 2, 3, 1, 3, 2], 2);
        actual.sort_unstable();
        let mut expected = vec![1, 2];
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_single_element_array_returns_the_array() {
        let mut actual = solve(vec![1000], 1);
        actual.sort_unstable();
        let mut expected = vec![1000];
        expected.sort_unstable();
        assert_eq!(actual, expected)
    }
}
