use std::env;
use std::process;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: bof <command>");
        process::exit(1);
    }

    match args[1].as_str() {
        "init" => init_command(),
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