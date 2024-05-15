use crate::build_by_target_pattern::rebuild_string_by_patterns;
use crate::search_by_pattern::search_file_names_by_pattern;
use crate::string_pattern::StringPattern;
use std::fs;
use std::path::Path;

/// Makes new full file names by using source name pattern and target name pattern.
/// For better understanding look [StringPattern]
/// and [mass_move_lib::build_by_target_pattern::build_by_target_pattern]
fn make_new_full_file_names(
    old_file_names: &Vec<String>,
    source_file_pattern: &StringPattern,
    target_path: &str,
    target_file_pattern: &str,
) -> Vec<String> {
    let mut new_full_file_names: Vec<String> = Vec::new();
    for file_name in old_file_names {
        let new_file_name =
            rebuild_string_by_patterns(file_name, source_file_pattern, target_file_pattern)
                .unwrap();
        let new_full_file_name = target_path.to_string() + "/" + &new_file_name;
        new_full_file_names.push(new_full_file_name);
    }
    new_full_file_names
}
/// If among given paths any path already exists function will return it. If not, then [None].
/// ```
/// # use std::fs;
/// # use std::fs::File;
/// use mass_move_lib::mass_move::find_existing_file;
///
/// fs::create_dir("src/exists");
///
/// assert_eq!(
///     find_existing_file(&vec!["not_exists".to_string(), "src/exists".to_string()]),
///     Some("src/exists")
/// );
///
/// assert_eq!(
///     find_existing_file(&vec!["not_exists1".to_string(), "not_exists2".to_string()]),
///     None
/// );
///
/// # fs::remove_dir("src/exists");
/// ```
pub fn find_existing_file(full_file_names: &Vec<String>) -> Option<&str> {
    for full_name in full_file_names {
        let path = Path::new(&full_name);
        if path.exists() {
            return Some(full_name);
        }
    }
    None
}

/// Takes all files that match source pattern (check [StringPattern]) then moves and
/// renames them according to the target pattern. If there already exist such files throws
/// an error if force_flag is false and subsistute them if true.
///
/// # Examples
///
/// ```
/// # use std::fs;
/// # use std::fs::File;
///
/// use mass_move_lib::mass_move::mass_move;
/// use mass_move_lib::search_by_pattern::search_file_names_by_pattern;
/// use mass_move_lib::string_pattern::StringPattern;
///
/// fs::create_dir("src/zxcexample1");
/// fs::create_dir("src/zxcexample2");
/// let file_example1 = File::create("src/zxcexample1/example.txt");
/// let file_example2 = File::create("src/zxcexample1/example_random_substring_.txt");
/// let pattern = StringPattern::new("*example.txt");
///
/// let _ = mass_move("src/zxcexample1/example*.txt", "src/zxcexample2/#1example.txt", false);
///
/// assert_eq!(
///     search_file_names_by_pattern("src/zxcexample2", &pattern),
///     Some(vec![
///         "_random_substring_example.txt".to_string(),
///         "example.txt".to_string(),  
///     ])
/// );
///
/// # fs::remove_file("src/zxcexample2/example.txt");
/// # fs::remove_file("src/zxcexample2/_random_substring_example.txt");
/// # fs::remove_dir("src/zxcexample1");
/// # fs::remove_dir("src/zxcexample2");
/// ```
pub fn mass_move(
    source_pattern: &str,
    target_pattern: &str,
    force_flag: bool,
) -> Result<(), String> {
    let source_pattern_rsplitted: Vec<&str> = source_pattern.rsplitn(2, '/').collect();
    if source_pattern_rsplitted.len() < 2 {
        return Err("Incorrect source_pattern".to_string());
    }
    let target_pattern_rsplitted: Vec<&str> = target_pattern.rsplitn(2, '/').collect();
    if target_pattern_rsplitted.len() < 2 {
        return Err("Incorrect target path".to_string());
    }

    let source_file_pattern: StringPattern = StringPattern::new(source_pattern_rsplitted[0]);
    let source_path = source_pattern_rsplitted[1];

    let target_file_pattern = target_pattern_rsplitted[0];
    let target_path = target_pattern_rsplitted[1];

    let matched_file_names = search_file_names_by_pattern(source_path, &source_file_pattern);
    if matched_file_names.is_none() {
        return Err(format!("Files for pattern {} not found", source_pattern));
    }

    let matched_file_names = matched_file_names.unwrap();
    let new_full_file_names: Vec<String> = make_new_full_file_names(
        &matched_file_names,
        &source_file_pattern,
        target_path,
        target_file_pattern,
    );

    if !force_flag {
        let existing_file = find_existing_file(&new_full_file_names);
        if existing_file.is_some() {
            return Err(format!(
                "Not able to replace existing file: {}",
                existing_file.unwrap()
            ));
        }
    }

    for file_index in 0..matched_file_names.len() {
        let old_full_file_name = source_path.to_string() + "/" + &matched_file_names[file_index];
        let result_of_moving = fs::rename(&old_full_file_name, &new_full_file_names[file_index]);
        if result_of_moving.is_err() {
            return Err(format!(
                "Couldn't move {} to {}",
                old_full_file_name, new_full_file_names[file_index]
            ));
        }
        println!(
            "{}//{} -> {}",
            source_path, matched_file_names[file_index], new_full_file_names[file_index]
        );
    }
    Ok(())
}
