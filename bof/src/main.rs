extern crate chrono;

use std::env;
use std::path::PathBuf;
use std::process;
use std::fs;
use sha1::{Sha1, Digest};
use std::os::unix::fs::MetadataExt;
use chrono::DateTime;
use chrono::offset::Utc;


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

/// Get bof dir from current path
fn get_bof_dir() -> PathBuf{
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let bof_dir = current_dir.join(".bof");

    bof_dir
}

fn init_command() {

    let bof_dir = get_bof_dir();

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

/// Creates the index directory that will contain the 
fn create_index_directory(index_directory: PathBuf) {
    if index_directory.exists() {
        println!("Index directory already exists.");
    } else {
        fs::create_dir(&index_directory).expect("Failed to create index directory");
        println!("Initialized empty index directory in {:?}", index_directory);
    }
}


fn index_command() {
    println!("Indexing the folder");

    let file_path = "./Cargo.toml";

    // TODO : create fn o compute hash (here won't work with directories)
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // create a Sha1 object
    let mut hasher = Sha1::new();

    hasher.update(contents);

    let hash_index = format!("{:x}", hasher.finalize());

    let (first_two, rest) = hash_index.split_at(2);

    println!("Hash index : {}", hash_index);
    println!("first two : {}", first_two);
    println!("rest : {}", rest);

    // create path for indexing

    let bof_directory = get_bof_dir();

    let index_directory = bof_directory.join(first_two);

    create_index_directory(index_directory);

    let metadata = fs::metadata(file_path).unwrap();

    let inode = metadata.ino();

    println!("inode : {}", inode);

    if metadata.is_file() {
        // Get dates

        let modification_system_time = metadata.modified().unwrap();

        let modification_datetime: DateTime<Utc> = modification_system_time.into();
        println!("Last modification date : {}", modification_datetime.format("%d/%m/%Y %T"));

        let creation_system_time = metadata.created().unwrap();

        let creation_datetime: DateTime<Utc> = creation_system_time.into();
        println!("Last modification date : {}", creation_datetime.format("%d/%m/%Y %T"));

        let size = metadata.size();

        println!("Document size in bytes: {}", size)
    } else if metadata.is_dir() {
        // Add information of the contained documents in the file

        println!("Adding indormation of the contained documents");

        let file_path_dir = fs::read_dir(file_path).unwrap();

        for path in file_path_dir {
            let unwrapped_path = path.unwrap().path();

            let path_name = unwrapped_path.display().to_string();

            println!("  NAME : {}", path_name);        

            if unwrapped_path.is_dir() {
                println!("  KIND : DIR");  
            } else if unwrapped_path.is_file() {
                println!("  KIND : FILE");  
            }

            let contents = fs::read_to_string(unwrapped_path).expect("Should have been able to read the file");

            let mut hasher = Sha1::new();

            hasher.update(contents);

            let hash_index = format!("{:x}", hasher.finalize());

            println!("  HAS : {}", hash_index);
        }

    }


}