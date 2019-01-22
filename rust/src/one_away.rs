/**
 * 1.5 One Away: There are three types of edits that can be performed on strings: insert a character, remove a character, or replace a character. Given two strings, write a function to check if they are one edit (or zero edits) away.
 * 
 * Example
 * pale, ple -> true
 * pales, pale -> true
 * pale, bale -> true
 * pale, bake -> false
 */
pub fn one_away(a: &str, b: &str) -> bool {
    let length_diff = (a.len() - b.len()) as i32;
    
    if length_diff == 0 {
        // Compare strings character-wise and count the differences.
        let num_chars_different = a.chars().zip(b.chars())
            .fold(0, |acc, (a, b)| if a == b { acc } else { acc + 1 });
        // Strings can differ by at most one character.
        num_chars_different <= 1
    } else if length_diff == 1 {
        // `a` is one character longer. Check that removing first conflicting char from `a` renders the strings equal.
        one_added(a, b)
    } else if length_diff == -1 {
        // `b` is one character longer. Check that removing first conflicting char from `b` renders the strings equal.
        one_added(b, a)
    } else {
        // If the strings differ in length by more than a character, they are not one away.
        false
    }
}

/**
 * Check that string `a` is the same as string `b` with one character inserted.
 */
fn one_added(a: &str, b: &str) -> bool {
    let mut a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();

    // Find the index of the first conflicting char.
    let added_char_index = a_chars.iter().enumerate()
        .position(|(i, a_char)| {
            match b_chars.get(i) {
                Some(b_char) => b_char != a_char,
                None => true,
            }
        })
        .expect("invariant violation: there must be at least one character different");

    // Remove the conflicting char.
    a_chars.remove(added_char_index);

    a_chars == b_chars
}

/**
 * 1.6 String Compression: Implement a method to perform basic string compression using the counts of repeated characters. For example, the string aabcccccaaa would become a2b1c5a3. If the "compressed" string would not become smaller than the original string, your method should return the original string. You can assume the string has only uppercase and lowercase letters (a-z).
 */
pub fn compress_string(string: &str) -> String {
    // If input empty, just return it.
    if string.len() == 0 {
        return String::from(string);
    }

    // Set up state machine.
    let mut output = String::new();
    let mut prev   = None;
    let mut count  = 0;

    // Create an iterator over characters.
    let mut it = string.chars();

    loop {
        // Get the next character, if any.
        let curr = it.next();
        match (curr, prev) {
            // The beginning of the sequence is the end of the sequence.
            // Hence the input is an empty string.
            (None, None) => break,

            // Beginning of the sequence
            (Some(ch), None) => {
                prev = Some(ch);
                count = 1;
            },

            // Partway through the sequence
            (Some(ch), Some(prev_ch)) => {
                if ch == prev_ch {
                    count += 1;
                } else {
                    output.push_str(&format!("{}{}", prev_ch, count));
                    prev = Some(ch);
                    count = 1;
                }
            },

            // End of the sequence
            (None, Some(prev_ch)) => {
                output.push_str(&format!("{}{}", prev_ch, count));
                break;
            },
        }
    }

    // Check that the compression algorithm actually compressed the string.
    if output.len() >= string.len() {
        String::from(string)
    } else {
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_away() {
        struct TestCase {
            a: &'static str,
            b: &'static str,
            expected: bool,
        }

        let cases = vec![
            TestCase {
                a: "pale",
                b: "ple",
                expected: true,
            },
            TestCase {
                a: "pales",
                b: "pale",
                expected: true,
            },
            TestCase {
                a: "pale",
                b: "bale",
                expected: true,
            },
            TestCase {
                a: "pale",
                b: "bake",
                expected: false,
            }
        ];
        for TestCase { a, b, expected } in cases {
            assert_eq!(one_away(a, b), expected, "one_away({}, {}) != {}", a, b, expected);
        }
    }

    #[test]
    fn test_string_compression() {
        struct TestCase {
            input: &'static str,
            expected: &'static str,
        }

        let cases = vec![
            TestCase {
                input: "aabcccccaaa",
                expected: "a2b1c5a3",
            },
            TestCase {
                input: "abc",
                expected: "abc",
            }
        ];

        for TestCase { input, expected } in cases {
            assert_eq!(compress_string(input), expected, "compress_string(\"{}\") != \"{}\"", input, expected);
        }
    }
}
