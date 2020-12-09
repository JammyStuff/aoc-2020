use std::collections::HashSet;

pub fn find_entries(entries: &[u64], sum: u64) -> Option<(u64, u64)> {
    let mut seen = HashSet::new();

    for i in entries {
        if let Some(rem) = sum.checked_sub(*i) {
            if seen.contains(&rem) {
                return Some((rem, *i));
            }
        }
        seen.insert(i);
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_entries_1() {
        let entries = vec![1721, 979, 366, 299, 675, 1456];
        let result = find_entries(&entries, 2020);
        assert_eq!(result, Some((1721, 299)));
    }

    #[test]
    fn test_find_entries_2() {
        let entries = vec![100, 200, 300, 500];
        let result = find_entries(&entries, 500);
        assert_eq!(result, Some((200, 300)));
    }

    #[test]
    fn test_find_entries_3() {
        let entries = vec![1, 2, 3, 4, 5];
        let result = find_entries(&entries, 100);
        assert_eq!(result, None);
    }
}