use crate::string_pattern::StringPattern;
use std::fs;
use std::string::String;

/// Searches file names in a given directory that match a given [StringPattern]
///
/// # Examples
///
/// ```
/// # use std::fs;
/// # use std::fs::File;
/// use mass_move_lib::string_pattern::StringPattern;
/// use mass_move_lib::search_by_pattern::search_file_names_by_pattern;
///
/// # fs::create_dir("src/example_dir");
/// let file_example1 = File::create("src/example_dir/example.txt");
/// let file_example2 = File::create("src/example_dir/example_anything.txt");
/// let pattern = StringPattern::new("example*.txt");
///
/// assert_eq!(
///     search_file_names_by_pattern("src/example_dir", &pattern),
///     Some(vec!["example.txt".to_string(), "example_anything.txt".to_string()])
/// );
///
/// # fs::remove_file("src/example_dir/example.txt");
/// # fs::remove_file("src/example_dir/example_anything.txt");
/// # fs::remove_dir("src/example_dir");
/// ```
pub fn search_file_names_by_pattern(
    path_to_directory: &str,
    pattern_file_name: &StringPattern,
) -> Option<Vec<String>> {
    let mut matching_file_names = Vec::new();
    if let Ok(directory_entries) = fs::read_dir(path_to_directory) {
        for entry in directory_entries.flatten() {
            if !entry.file_type().unwrap().is_file() {
                continue;
            }
            let file_name = entry.file_name().to_string_lossy().to_string();
            if pattern_file_name.is_matches(&file_name) {
                matching_file_names.push(file_name)
            }
        }
    } else {
        return None;
    }
    if matching_file_names.is_empty() {
        return None;
    }
    matching_file_names.sort();
    Some(matching_file_names)
}

#[cfg(test)]
mod tests {
    use crate::{search_by_pattern::search_file_names_by_pattern, string_pattern::StringPattern};

    use std::fs::File;
    use std::io::Write;
    use tempfile::{self, TempDir};

    #[test]
    fn test_empty_directory() {
        let pattern: StringPattern = StringPattern::new("*");
        let temporary_directory = TempDir::new().unwrap();
        let directory_path = temporary_directory.path().to_str().unwrap();
        assert_eq!(search_file_names_by_pattern(directory_path, &pattern), None);
    }

    #[test]
    fn test_default() {
        let pattern: StringPattern = StringPattern::new("some_*_filename.*");
        let file_names: Vec<String> = vec![
            "some_A_filename.bin".to_string(),
            "some_A_filename.jpg".to_string(),
            "some_B_filename.bin".to_string(),
            "some_B_filename.jpg".to_string(),
        ];
        let temporary_directory = TempDir::new().unwrap();
        let directory_path = temporary_directory.path().to_str().unwrap();
        for file_name in &file_names {
            let file_path = format!("{}/{}", directory_path, file_name);
            let mut file = File::create(&file_path).unwrap();
            file.write_all(b"hihihihi").unwrap();
        }
        assert_eq!(
            search_file_names_by_pattern(directory_path, &pattern).unwrap(),
            file_names
        );
    }

    #[test]
    fn test_default_with_irrelevant_files() {
        let pattern: StringPattern = StringPattern::new("some_*_filename.*");
        let relevant_file_names: Vec<String> = vec![
            "some_A_filename.bin".to_string(),
            "some_A_filename.jpg".to_string(),
            "some_B_filename.bin".to_string(),
            "some_B_filename.jpg".to_string(),
        ];
        let irrelevant_file_names = vec![
            "*.jpg".to_string(),
            "some_.bin".to_string(),
            "some_*_filename".to_string(),
        ];
        let temporary_directory = TempDir::new().unwrap();
        let directory_path = temporary_directory.path().to_str().unwrap();
        for file_name in &relevant_file_names {
            let file_path = format!("{}/{}", directory_path, file_name);
            let mut file = File::create(&file_path).unwrap();
            file.write_all(b"hihihihi").unwrap();
        }
        for file_name in &irrelevant_file_names {
            let file_path = format!("{}/{}", directory_path, file_name);
            let mut file = File::create(&file_path).unwrap();
            file.write_all(b"hihihihi").unwrap();
        }
        assert_eq!(
            search_file_names_by_pattern(directory_path, &pattern).unwrap(),
            relevant_file_names
        );
    }
}
