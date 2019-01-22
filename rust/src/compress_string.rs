
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
