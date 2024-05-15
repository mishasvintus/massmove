use regex::Regex;

/// An auxiliary structure that helps to work with a type of string patterns that look
/// like a common string but '*' symbols means that any substring can be placed at that place.
pub struct StringPattern {
    pub slices: Vec<String>,
    pub regex: Regex,
}

impl StringPattern {
    ///Returns new [StringPattern] formed by a string.
    pub fn new(raw_string_pattern: &str) -> Self {
        let pattern_slices = Self::get_pattern_slices(raw_string_pattern);
        let pattern_regex = Self::get_regex(&pattern_slices);
        Self {
            slices: pattern_slices,
            regex: pattern_regex,
        }
    }

    /// Auxiliary function that is needed to create a [StringPattern] slices field when
    /// making new object.
    fn get_pattern_slices(raw_string_pattern: &str) -> Vec<String> {
        let mut pattern_slices: Vec<String> = Vec::new();
        let mut last_index_of_slices: usize = 0;
        for letter in raw_string_pattern.chars() {
            if letter == '*' {
                pattern_slices.push(String::new());
                if pattern_slices.len() == 1 {
                    last_index_of_slices += 1;
                } else {
                    last_index_of_slices += 2;
                }
                continue;
            }
            if last_index_of_slices as i32 > (pattern_slices.len() as i32 - 1) {
                pattern_slices.push("".to_string());
            }
            pattern_slices[last_index_of_slices] += &letter.to_string();
        }
        pattern_slices
    }
    /// Auxiliary function that is needed to create a [StringPattern] regex field when
    /// making new object
    fn get_regex(pattern_slices: &[String]) -> Regex {
        let mut regex_pattern_string = String::new();
        for pattern_slice in pattern_slices.iter() {
            regex_pattern_string.push_str(pattern_slice);
            if pattern_slice.is_empty() {
                regex_pattern_string.push_str(r".*?");
            }
        }
        regex_pattern_string = format!("^{}$", regex_pattern_string);
        Regex::new(&regex_pattern_string).unwrap()
    }

    /// Returns true if and only if a given string matches the pattern.
    /// # Examples
    /// ```
    /// use mass_move_lib::string_pattern::StringPattern;
    ///
    /// let pattern: StringPattern = StringPattern::new("ab*cde*");
    ///
    /// assert_eq!(pattern.is_matches(""), false);
    /// assert_eq!(pattern.is_matches("ab"), false);
    /// assert_eq!(pattern.is_matches("abcde"), true);
    /// assert_eq!(pattern.is_matches("ab123cde321"), true);
    /// assert_eq!(pattern.is_matches("ab123cde"), true);
    /// assert_eq!(pattern.is_matches("abababcdecdecdef"), true);
    /// ```
    pub fn is_matches(&self, string_to_check: &str) -> bool {
        self.regex.is_match(string_to_check)
    }

    /// Returns [Option<Vec<&str>>] which is [None] if a given string doesn't match the pattern.
    /// If the string matches function will return a vector of substrings that hide
    /// behind '*' symbols of string pattern.
    ///
    /// # Examples
    /// ```
    /// use mass_move_lib::string_pattern::StringPattern;
    ///
    /// let pattern: StringPattern = StringPattern::new("ab*cde*");
    ///
    /// assert_eq!(pattern.get_generic_parts(""), None);
    /// assert_eq!(pattern.get_generic_parts("ab"), None);
    /// assert_eq!(pattern.get_generic_parts("abcde").unwrap(), vec!["", ""]);
    /// assert_eq!(pattern.get_generic_parts("ab123cde321").unwrap(), vec!["123", "321"]);
    /// assert_eq!(pattern.get_generic_parts("ab123cde").unwrap(), vec!["123", ""]);
    /// assert_eq!(pattern.get_generic_parts("abababcdecdecdef").unwrap(), vec!["abab", "cdecdef"]);
    /// ```
    pub fn get_generic_parts<'a>(&self, string_to_decompose: &'a str) -> Option<Vec<&'a str>> {
        if !self.is_matches(string_to_decompose) {
            return None;
        }
        let mut generic_parts: Vec<&'a str> = Vec::new();
        let mut residual_string = string_to_decompose;
        for pattern_slice in self.slices.iter() {
            if pattern_slice.is_empty() {
                generic_parts.push("");
                continue;
            }
            let splitted_residual_string = residual_string.split_once(pattern_slice).unwrap();
            if !splitted_residual_string.0.is_empty() {
                generic_parts.pop();
                generic_parts.push(splitted_residual_string.0);
            }
            residual_string = splitted_residual_string.1
        }
        if !residual_string.is_empty() {
            generic_parts.pop();
            generic_parts.push(residual_string);
        }
        Some(generic_parts)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod test_is_matches {
        use crate::string_pattern::StringPattern;

        #[test]
        fn test_without_stars() {
            let pattern: StringPattern = StringPattern::new("abc");
            assert_eq!(pattern.is_matches(""), false);
            assert_eq!(pattern.is_matches("abc"), true);
            assert_eq!(pattern.is_matches("abcabc"), false);
            assert_eq!(pattern.is_matches("ab"), false);
        }

        #[test]
        fn test_only_star() {
            let pattern: StringPattern = StringPattern::new("*");
            assert_eq!(pattern.is_matches(""), true);
            assert_eq!(pattern.is_matches("abc"), true);
            assert_eq!(pattern.is_matches("abcabc"), true);
            assert_eq!(pattern.is_matches("ab"), true);
        }

        #[test]
        fn test_empty_pattern() {
            let pattern: StringPattern = StringPattern::new("");
            assert_eq!(pattern.is_matches(""), true);
            assert_eq!(pattern.is_matches("abc"), false);
            assert_eq!(pattern.is_matches("abcabc"), false);
            assert_eq!(pattern.is_matches("ab"), false);
        }

        #[test]
        fn test_with_two_stars() {
            let pattern: StringPattern = StringPattern::new("ab*cde*f");
            assert_eq!(pattern.is_matches(""), false);
            assert_eq!(pattern.is_matches("ab1234cde45234f"), true);
            assert_eq!(pattern.is_matches("ab"), false);
            assert_eq!(pattern.is_matches("abcdef"), true);
            assert_eq!(pattern.is_matches("abababcdecdecdef"), true);
        }

        #[test]
        fn test_stars_at_the_end() {
            let pattern: StringPattern = StringPattern::new("ab*cde*");
            assert_eq!(pattern.is_matches(""), false);
            assert_eq!(pattern.is_matches("ab12a34cde45v234f"), true);
            assert_eq!(pattern.is_matches("ab"), false);
            assert_eq!(pattern.is_matches("abcdef"), true);
            assert_eq!(pattern.is_matches("abababcdecdecdef"), true);
        }
    }

    #[cfg(test)]
    mod test_get_generic_parts {
        use crate::string_pattern::StringPattern;

        #[test]
        fn test_empty() {
            let pattern: StringPattern = StringPattern::new("");
            let empty_vector: Vec<&str> = vec![];
            assert_eq!(pattern.get_generic_parts("").unwrap(), empty_vector);
        }

        #[test]
        fn test_only_stars() {
            let pattern = StringPattern::new("*");
            assert_eq!(pattern.get_generic_parts("").unwrap(), vec![""]);
            assert_eq!(pattern.get_generic_parts("abc").unwrap(), vec!["abc"]);

            let pattern = StringPattern::new("**");
            assert_eq!(pattern.get_generic_parts("").unwrap(), vec!["", ""]);
            assert_eq!(pattern.get_generic_parts("abc").unwrap(), vec!["", "abc"]);

            let pattern = StringPattern::new("***");
            assert_eq!(pattern.get_generic_parts("").unwrap(), vec!["", "", ""]);
            assert_eq!(
                pattern.get_generic_parts("abc").unwrap(),
                vec!["", "", "abc"]
            );
        }

        #[test]
        fn test_without_stars() {
            let pattern: StringPattern = StringPattern::new("abc");
            let empty_vector: Vec<&str> = vec![];

            assert_eq!(pattern.get_generic_parts(""), None);
            assert_eq!(pattern.get_generic_parts("abcabc"), None);
            assert_eq!(pattern.get_generic_parts("abc").unwrap(), empty_vector);
        }

        #[test]
        fn test_inserted_stars() {
            let pattern: StringPattern = StringPattern::new("ab*cde*");
            assert_eq!(pattern.get_generic_parts("abcde").unwrap(), vec!["", ""]);
            assert_eq!(
                pattern.get_generic_parts("ab12a34cde45v234f").unwrap(),
                vec!["12a34", "45v234f"]
            );
            assert_eq!(pattern.get_generic_parts("abcdef").unwrap(), vec!["", "f"]);

            let pattern: StringPattern = StringPattern::new("ab*cde*f");
            assert_eq!(pattern.get_generic_parts("abcdef").unwrap(), vec!["", ""]);
            assert_eq!(
                pattern.get_generic_parts("ab12a34cde45v234f").unwrap(),
                vec!["12a34", "45v234"]
            );
            assert_eq!(pattern.get_generic_parts("abcde1f").unwrap(), vec!["", "1"]);

            let pattern = StringPattern::new("*abc");
            assert_eq!(pattern.get_generic_parts("abc").unwrap(), vec![""]);
            assert_eq!(pattern.get_generic_parts("1abc").unwrap(), vec!["1"]);
        }
    }
}
