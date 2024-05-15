use crate::string_pattern::StringPattern;
use regex::{Captures, Regex};
/// Returns a new string with given strings to put in relevant places in given pattern.
/// These places look like /subsistution_pattern_prefix + num/ where num is index of string from
/// strings_to_put. Indexing of strings to put starts with 1. If num is out of bound,
/// function throws an error
///
/// # Example
/// ```
/// use mass_move_lib::build_by_target_pattern::build_by_target_pattern;
///
/// let put: Vec<&str> = vec!["A", "B", "C"];
/// let target_pattern: &str = "a#3b#2c#1";
/// assert_eq!(build_by_target_pattern(&put, target_pattern, "#"), "aCbBcA");
///
/// // Causes an error
/// // let empty: Vec<&str> = vec![];
/// // let _ = build_by_target_pattern(&empty, "#1#2#3", "#");
///
/// ```
pub fn build_by_target_pattern(
    strings_to_put: &Vec<&str>,
    target_pattern: &str,
    subsistution_pattern_prefix: &str,
) -> String {
    let subsistution_pattern: String = subsistution_pattern_prefix.to_string() + r"(\d+)";
    let substitution_pattern = Regex::new(subsistution_pattern.as_str()).unwrap();

    let put_strings_closure = |string_index_capture: &Captures| {
        let mut string_index = string_index_capture
            .get(1)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();

        if string_index <= strings_to_put.len() && string_index > 0 {
            string_index -= 1;
            strings_to_put[string_index].to_string()
        } else {
            panic!("Index of string to put is out of bound")
        }
    };

    let result = substitution_pattern
        .replace_all(target_pattern, put_strings_closure)
        .to_string();
    result
}

/// Returns a new string that is got by extracting generic parts from old string with [StringPattern]
/// and placing them in target pattern
///
/// #Examples
/// ```
/// # use mass_move_lib::string_pattern::StringPattern;
/// use mass_move_lib::build_by_target_pattern::rebuild_string_by_patterns;
///
/// assert_eq!(
///     rebuild_string_by_patterns("abcdef", &StringPattern::new("a*bcd*f"), "#1#2#1").unwrap(),
///     "e"
/// );
/// assert_eq!(
///     rebuild_string_by_patterns("abcdef", &StringPattern::new("a*cdef"), "123_#1_123")
///         .unwrap(),
///     "123_b_123"
/// );
/// ```
pub fn rebuild_string_by_patterns(
    string_to_rebuild: &str,
    string_pattern: &StringPattern,
    target_pattern: &str,
) -> Option<String> {
    let generic_parts = string_pattern.get_generic_parts(string_to_rebuild);
    generic_parts.as_ref()?;

    let generic_parts = generic_parts.unwrap();
    Some(build_by_target_pattern(&generic_parts, target_pattern, "#"))
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod test_build_by_target_pattern {
        use crate::build_by_target_pattern::build_by_target_pattern;

        #[test]
        fn test_empty() {
            let put: Vec<&str> = vec![];
            assert_eq!(build_by_target_pattern(&put, "", "#"), "");
        }

        #[test]
        #[should_panic(expected = "Index of string to put is out of bound")]
        fn test_out_of_index() {
            let empty_vector: Vec<&str> = vec![];
            let _ = build_by_target_pattern(&empty_vector, "#1#2#3", "#");
        }

        #[test]
        fn test_few_cases() {
            let put: Vec<&str> = vec!["A", "B", "C"];
            assert_eq!(build_by_target_pattern(&put, "a#1b#2c#1", "#"), "aAbBcA");
            assert_eq!(build_by_target_pattern(&put, "a#3b#2c#1", "#"), "aCbBcA");

            let put: Vec<&str> = vec!["A", "B", "C"];
            assert_eq!(build_by_target_pattern(&put, "#1#2#1", "#"), "ABA");

            let put: Vec<&str> = vec!["A", "B", "C"];
            assert_eq!(build_by_target_pattern(&put, "", "#"), "");
        }
    }

    #[cfg(test)]
    mod test_rebuild_string_by_patterns {
        use crate::build_by_target_pattern::rebuild_string_by_patterns;
        use crate::string_pattern::StringPattern;

        #[test]
        fn test_empty() {
            assert_eq!(
                rebuild_string_by_patterns("", &StringPattern::new(""), "").unwrap(),
                ""
            );
        }

        #[test]
        fn test_normal() {
            assert_eq!(
                rebuild_string_by_patterns("abcdef", &StringPattern::new("a*f"), "#1").unwrap(),
                "bcde"
            );
            assert_eq!(
                rebuild_string_by_patterns("abcdef", &StringPattern::new("a*bcd*f"), "#1#2#1")
                    .unwrap(),
                "e"
            );
            assert_eq!(
                rebuild_string_by_patterns("abcdef", &StringPattern::new("a*cdef"), "123_#1_123")
                    .unwrap(),
                "123_b_123"
            );
        }
    }
}
