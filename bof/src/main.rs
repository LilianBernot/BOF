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
        "showdir" => showdir_command("./"),
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

fn showdir_command(current_path:&str) {

    let current_path_unwrap = fs::read_dir(current_path).unwrap();

    for path in current_path_unwrap {
        let unwrapped_path = path.unwrap().path();

        let path_name = unwrapped_path.display().to_string();

        println!("{}", path_name);        

        if unwrapped_path.is_dir() {

            if path_name == "./target/debug" {
                println!("\t...");
            } else {
                showdir_command(&path_name);
            }
        }
    }
}