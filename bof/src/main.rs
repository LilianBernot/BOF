extern crate chrono;

use std::env;
use std::fs::Metadata;
use std::os::unix::fs::FileExt;
use std::path::PathBuf;
use std::process;
use std::fs;
use sha1::{Sha1, Digest};
use std::os::unix::fs::MetadataExt;
use chrono::DateTime;
use chrono::offset::Utc;
use std::fs::File;
use std::io::{self, Write};


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: bof <command>");
        process::exit(1);
    }

    match args[1].as_str() {
        "init" => init_command(),
        "showdir" => showdir_command("./", 0),
        "index" => index_command("./src"),
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
fn create_index_directory(index_directory: &PathBuf) {
    if index_directory.exists() {
        println!("Index directory already exists.");
    } else {
        fs::create_dir(&index_directory).expect("Failed to create index directory");
        println!("Initialized empty index directory in {:?}", index_directory);
    }
}

/// This function hashes a file content and returns the hash
fn hash_file(file_path : &str) -> String {
    let contents = fs::read_to_string(file_path).expect("Could not read the file.");

    // create a Sha1 object
    let mut hasher = Sha1::new();

    hasher.update(contents);

    format!("{:x}", hasher.finalize())
}

/// Creates the hash of a folder
///
/// # Returns 
/// 
/// * generated hash
/// * string to write to the indew file 
fn hash_folder(folder_path: &str) -> (String, String)  {

    let mut entries: Vec<(String, String, &str)> = Vec::new();
    // Will contain (path_name, hash, kind) for each contained element

    let folder_path_dir = fs::read_dir(folder_path).unwrap();

    for path in folder_path_dir {
        let unwrapped_path = path.unwrap().path();

        let path_name = unwrapped_path.display().to_string();



        if unwrapped_path.is_file() {
            let file_hash = hash_file(&path_name);
            entries.push((path_name, file_hash, "FILE"));
        } else if unwrapped_path.is_dir() {
            let (folder_hash, _data) = hash_folder(&path_name);
            entries.push((path_name, folder_hash, "DIRECTORY"));
        }
    }

    // Concatenate name and hash for entries
    
    let mut hasher = Sha1::new();
    let mut data_to_write = String::new();

    for (name, hash, kind) in entries {
        data_to_write.push_str("NAME : ");
        data_to_write.push_str(&name);
        hasher.update(name);
        data_to_write.push_str("\nKIND : ");
        data_to_write.push_str(kind);
        data_to_write.push_str("\nHASH : ");
        data_to_write.push_str(&hash);
        data_to_write.push_str("\n");
        hasher.update(hash);
    }

    (format!("{:x}", hasher.finalize()), data_to_write)
}

/// Concatenates the data to write in the index file when path is a file
/// 
/// # Returns
/// 
/// * data to write to the index file
fn get_index_file_data(metadata:Metadata) -> String {
    let mut data_to_write: String = String::from("\n");

    if metadata.is_file() {
        // Creation time
        let creation_datetime: DateTime<Utc> =  metadata.created().unwrap().into();
        let createion_time = format!("{}", creation_datetime.format("%d/%m/%Y %T"));
        data_to_write.push_str("CREATION TIME : ");
        data_to_write.push_str(&createion_time);

        // Modification time
        let modification_datetime: DateTime<Utc> = metadata.modified().unwrap().into();
        let modification_time = format!("{}", modification_datetime.format("%d/%m/%Y %T"));
        data_to_write.push_str("\nLAST MODIFICATION TIME : ");
        data_to_write.push_str(&modification_time);

        // Size
        let size: String = metadata.size().to_string();
        data_to_write.push_str("\nSIZE IN BYTES : ");
        data_to_write.push_str(&size);
        data_to_write.push_str("\n");
    }
    return data_to_write
}

fn create_index_file(hash_index: String, metadata: Metadata) -> File {
    // Create index directory
    let bof_directory = get_bof_dir();

    let (first_two, rest) = hash_index.split_at(2);

    let index_directory = bof_directory.join(first_two);

    create_index_directory(&index_directory);

    // Create index file
    let index_file_name = format!("{}.txt", rest);

    let index_file_path = index_directory.join(index_file_name);

    let mut index_file = File::create(index_file_path).unwrap();

    // Write index file
    write!(index_file, "HASH : ").unwrap();
    writeln!(index_file, "{}", hash_index).unwrap();

    let inode = metadata.ino();
    write!(index_file, "INODE : ").unwrap();
    writeln!(index_file, "{}", inode).unwrap();

    return index_file
}

fn index_command(current_path:&str) {
    println!("Indexing the following element : {}", current_path);

    let metadata = fs::metadata(current_path).unwrap();

    let hash_index:String;
    let data_to_write:String;
    if metadata.is_file() {
        hash_index = hash_file(&current_path);
        data_to_write = get_index_file_data(metadata.clone());
    } else if metadata.is_dir() {
        // Calling the function for each element under this folder
        let folder_path_dir = fs::read_dir(current_path).unwrap();
        for path in folder_path_dir {
            let unwrapped_path = path.unwrap().path();
            let path_name = unwrapped_path.display().to_string();
            index_command(&path_name)
        }

        (hash_index, data_to_write) = hash_folder(&current_path);
    } else {
        hash_index = String::from("");
        data_to_write = String::from("");
    }

    let mut index_file = create_index_file(hash_index, metadata.clone());

    writeln!(index_file, "{}", data_to_write).unwrap();
}