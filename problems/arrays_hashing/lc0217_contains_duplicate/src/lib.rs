use std::collections::HashSet;
pub fn solve(nums: Vec<i32>) -> bool {
    let mut set = HashSet::new();
    for num in nums {
        if set.contains(&num) {
            return true;
        } else {
            set.insert(num);
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duplicacy_returns_true() {
        assert!(solve(vec![1, 2, 2, 3]));
    }

    #[test]
    fn test_all_unique_returns_false() {
        assert_eq!(solve(vec![1, 2, 3, 4]), false);
    }
}
