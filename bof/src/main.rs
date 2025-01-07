use std::env;
use std::path::PathBuf;
use std::process;
use std::fs;
use sha1::{Sha1, Digest};


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: bof <command>");
        process::exit(1);
    }

    match args[1].as_str() {
        "init" => init_command(),
        "showdir" => showdir_command("./", 0),
        "index" => index_command(),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            process::exit(1);
        }
    }
}

fn init_command() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let bof_dir = current_dir.join(".bof");

    if bof_dir.exists() {
        println!("The .bof directory already exists.");
    } else {
        fs::create_dir(&bof_dir).expect("Failed to create .bof directory");
        println!("Initialized empty .bof directory in {:?}", bof_dir);
    }
}

/// Recursively displays the directory structure starting from a given path.
///
/// This function traverses the directory tree starting at the specified `current_path`,
/// printing the directory and file names indented to reflect their depth in the tree.
/// For subdirectories, the function calls itself recursively to continue the traversal.
///
/// # Arguments
///
/// * `current_path` - A string slice representing the path of the directory to start from.
/// * `depth` - The current depth level, used to calculate the indentation for display purposes.
///
/// # Panics
///
/// This function will panic if it encounters an error reading the directory specified by `current_path`.
///
/// # Examples
///
/// ```
/// showdir_command("./my_directory", 0);
/// ```
///
/// This will display the structure of `./my_directory` and its subdirectories.
fn showdir_command(current_path:&str, depth: usize) {

    let current_path_unwrap = fs::read_dir(current_path).unwrap();

    let display_prefix = "\t".repeat(depth);

    for path in current_path_unwrap {
        let unwrapped_path = path.unwrap().path();

        let path_name = unwrapped_path.display().to_string();

        println!("{} {}", display_prefix, path_name);        

        if unwrapped_path.is_dir() {

            if path_name == "./target/debug" {
                println!("{}\t...", display_prefix);
            } else {
                showdir_command(&path_name, depth+1);
            }
        }
    }
}


fn index_command() {
    println!("Indexing the folder");

    let file_path = "./Cargo.toml";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // create a Sha1 object
    let mut hasher = Sha1::new();

    hasher.update(contents);

    let hash_index = format!("{:x}", hasher.finalize());

    let (first_two, rest) = hash_index.split_at(2);

    println!("Hash index : {}", hash_index);
    println!("first two : {}", first_two);
    println!("rest : {}", rest)
}