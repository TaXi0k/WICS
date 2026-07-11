use std::env;
use std::fs;

mod file;

fn main() {
    println!("Hello, world!");

    //ARGUMENTS------------------------------------
    //Collect arguments
    let input: Vec<String> = env::args().collect();

    //Get 1st argument (path)
    let path: String = input[1].clone();

    //Get other arguments (non-path ones)
    let args: Vec<String> = input[2..].to_vec();
    //---------------------------------------------


    //Get all files in specified directory
    let files: Vec<String> = fs::read_dir(&path)
        .unwrap() 
        .map(|entry| entry.unwrap()
            .file_name()
            .into_string()
            .unwrap())
        .collect();


    dbg!(&input);
    dbg!(&path);
    dbg!(&args);
    dbg!(&files);

    println!("\n\n\n\n\n Test of file::check_file():\n===================================\n");
    for file in files {
        println!("\n\n Output for file {}:", &file);
        println!("{}", file::check_file(&path, &file));
    }
}
