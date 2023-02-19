use std::env;
use std::fs;
use std::fs::metadata;
use std::str::FromStr;

fn print_directory_content(dir_path: &str, layer: Option<i32>) {
    let paths = fs::read_dir(dir_path).expect("ERROR: Cannot open given directory.");

    let current_layer = layer.unwrap_or(0);

    for path in paths {
        let path_string =
            String::from_str(path.as_ref().unwrap().path().to_str().unwrap()).unwrap();
        let md = metadata(path_string.clone());
        if md.is_err() {
            continue;
        }
        let md = md.unwrap();
        let filename = String::from_str(path.unwrap().file_name().to_str().unwrap()).unwrap();
        for _ in 0..current_layer {
            print!("| ");
        }
        println!("{}", filename);
        if md.is_dir() {
            print_directory_content(&path_string, Some(current_layer + 1));
        }
    }

    if current_layer != 0 {
        for _ in 0..(current_layer - 1) {
            print!("| ");
        }
        println!("╰---");
    }
}

fn count_and_print_files(dir_path: &str) {
    let mut files_count = 0;
    let mut folders_count = 0;

    count_recursive(dir_path, &mut files_count, &mut folders_count);

    println!("Files: {}, Folders: {}", files_count, folders_count);

    fn count_recursive(dir_path: &str, files_count: &mut i32, folders_count: &mut i32) {
        let directory = fs::read_dir(dir_path).expect("ERROR: Cannot open given directory.");
        for entry in directory {
            let path_string =
                String::from_str(entry.as_ref().unwrap().path().to_str().unwrap()).unwrap();
            let md = metadata(path_string.clone());
            if md.is_err() {
                continue;
            }
            let md = md.unwrap();

            if md.is_dir() {
                *folders_count += 1;
                count_recursive(&path_string, files_count, folders_count);
            } else {
                *files_count += 1;
            }
        }
    }
}

fn main() {
    let current_dir = String::from_str(env::current_dir().unwrap().to_str().unwrap()).unwrap();
    println!("Path: {}\n", current_dir);
    print_directory_content(&current_dir, None);
    count_and_print_files(&current_dir);
}
