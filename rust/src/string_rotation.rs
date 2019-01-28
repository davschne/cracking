/**
 * 1.9 String Rotation: Assume you have a method `isSubstring` which checks if one word is a substring of another. Given two strings, `s1` and `s2`, write code to check if `s2` is a rotation of `s1` using only one call to `isSubstring` (e.g. "waterbottle" is a rotation of "erbottlewat").
 */
pub fn is_rotation(s1: &str, s2: &str) -> bool {
    is_substring(&s1.repeat(2), s2)
}

fn is_substring(haystack: &str, needle: &str) -> bool {
    haystack.contains(needle)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_is_rotation() {
        assert_eq!(is_rotation("waterbottle", "erbottlewat"), true);
        assert_eq!(is_rotation("waterbottle", "rebottlewat"), false);
    }
}
