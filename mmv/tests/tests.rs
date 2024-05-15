#[cfg(test)]
mod test_mass_move {
    use mass_move_lib::mass_move::mass_move;
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use tempfile::{self, TempDir};

    #[test]
    fn test_empty() {
        assert_eq!(
            mass_move("", "", false),
            Err("Incorrect source_pattern".to_string())
        );
    }

    #[test]
    fn test_same_names() {
        let file_names: Vec<String> = vec![
            "some_A_filename.bin".to_string(),
            "some_A_filename.jpg".to_string(),
            "some_B_filename.bin".to_string(),
            "some_B_filename.jpg".to_string(),
        ];
        let mut new_file_names: Vec<String> = vec![
            "some_A_filename.bin".to_string(),
            "some_A_filename.jpg".to_string(),
            "some_B_filename.bin".to_string(),
            "some_B_filename.jpg".to_string(),
        ];
        new_file_names.sort();

        let temporary_directory1 = TempDir::new().unwrap();
        let directory_path1 = temporary_directory1.path().to_str().unwrap();

        for file_name in &file_names {
            let file_path = format!("{}/{}", directory_path1, file_name);
            let mut file = File::create(&file_path).unwrap();
            file.write_all(b"hihihihi").unwrap();
        }

        let res = mass_move(
            &(directory_path1.to_string() + "/*"),
            &(directory_path1.to_string() + "/#1"),
            true,
        );
        assert!(res.is_ok());
        let mut moved_files: Vec<String> = Vec::new();
        for entry in fs::read_dir(directory_path1).unwrap() {
            let entry = entry.unwrap();
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy().to_string();
            moved_files.push(file_name_str);
        }
        moved_files.sort();
        assert_eq!(new_file_names, moved_files);
    }

    #[test]
    fn test_default() {
        let file_names: Vec<String> = vec![
            "some_A_filename.bin".to_string(),
            "some_A_filename.jpg".to_string(),
            "some_B_filename.bin".to_string(),
            "some_B_filename.jpg".to_string(),
        ];
        let mut new_file_names: Vec<String> = vec![
            "changed_A_filename.bin".to_string(),
            "changed_A_filename.jpg".to_string(),
            "changed_B_filename.bin".to_string(),
            "changed_B_filename.jpg".to_string(),
        ];
        new_file_names.sort();

        let temporary_directory1 = TempDir::new().unwrap();
        let directory_path1 = temporary_directory1.path().to_str().unwrap();
        let temporary_directory2 = TempDir::new().unwrap();
        let directory_path2 = temporary_directory2.path().to_str().unwrap();

        for file_name in &file_names {
            let file_path = format!("{}/{}", directory_path1, file_name);
            let mut file = File::create(&file_path).unwrap();
            file.write_all(b"hihihihi").unwrap();
        }

        let res: Result<(), String> = mass_move(
            &(directory_path1.to_string() + "/some_*_filename.*"),
            &(directory_path2.to_string() + "/changed_#1_filename.#2"),
            false,
        );
        assert!(res.is_ok());

        let mut moved_files: Vec<String> = Vec::new();
        for entry in fs::read_dir(directory_path2).unwrap() {
            let entry = entry.unwrap();
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy().to_string();
            moved_files.push(file_name_str);
        }

        moved_files.sort();
        assert_eq!(new_file_names, moved_files);
    }
}
