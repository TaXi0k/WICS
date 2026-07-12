use std::env;
use std::fs;
use std::sync::OnceLock;
use tools::*;
use owo_colors::OwoColorize;

use crate::api::QueryError;
use crate::api::QueryResult;
use crate::tools::LogType::Error;
use crate::tools::LogType::Plain;

mod api;
mod file;
mod tools;

pub struct AppConfig {
    pub log: bool,
    pub list_ok: bool,
} 

pub static CONFIG: OnceLock<AppConfig> = OnceLock::new();

fn main() {
    println!("Hello, world!");

    let mut log_flag = false;
    let mut list_ok_flag = false;

    //ARGUMENTS------------------------------------
    //Collect arguments
    let input: Vec<String> = env::args().collect();

    //Get 1st argument (path)
    let path: String = input[1].clone();

    //Get other arguments (non-path ones)
    let args: Vec<String> = input[2..].to_vec();

    if args.contains(&"-log".to_string()) {
        log_flag = true;
    }
    if args.contains(&"-ok".to_string()) {
        list_ok_flag = true;
    }

    CONFIG.set(AppConfig {
        log: log_flag,
        list_ok: list_ok_flag,
    }).ok().expect("Failed to initialize global configuration");

    //---------------------------------------------

    let config = CONFIG.get().expect(&format!("⛔  {}", "Config not initialized!".red()));


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

    if config.log {
        println!("ℹ️  {}", "Found following files:".bright_blue());
        dbg!(&files);
    }

    let mut ok_mods: Vec<String> = vec![];
    let mut bad_mods: Vec<String> = vec![];
    let mut unknown_mods: Vec<String> = vec![];
    let mut non_mods_and_directories: Vec<String> = vec![];

    log("\n\n", Plain);
    for file in files {
        log(&format!("\nCheckging file: {file}:"), Plain);
        
        match file::check_file(&path, &file) {
            Some(result) => {
                match result {
                    file::CheckResult::ServerOk => ok_mods.push(file),
                    file::CheckResult::ServerBad => bad_mods.push(file),
                    file::CheckResult::Unknown => unknown_mods.push(file),
                    file::CheckResult::Directory => non_mods_and_directories.push(file),
                    file::CheckResult::NonJAR => non_mods_and_directories.push(file),
                }
            },
            None => {
                log(&format!("Checking information about file {file} failed at some point, use -log flag for more information"), Error);
            },
        }
    }

    println!("\n\n\n==={{ {} }}===", "RESULTS".bold().purple());
    
    if config.list_ok {
        if !ok_mods.is_empty() {
            println!("✅  {}", "Server-safe mods:".green());
            for entry in ok_mods {
                println!("{} {}", "•".bright_green(), entry);
            }
        } else {
            println!("✅  {}", "Server-safe mods:".green());
            println!("  Found no mods that are server-safe (check unknown mods tho (if there are any))");
        }
        println!();
    }

    if !bad_mods.is_empty() {
        println!("❌  {}", "Mods not supported on server:".red());
        for entry in bad_mods {
            println!("{} {}", "•".bright_red(), entry);
        }
    } else {
        println!("❌  {}", "Mods not supported on server:".red());
        println!("  Found no mods that are not supported on server (check unknown mods tho (if there are any))");
    }
    println!();

    if !unknown_mods.is_empty() {
        println!("🟡  {}", "Mods that had not specified server_side support on Modrinth (unknown mods):".yellow());
        for entry in unknown_mods {
            println!("{} {}", "•".bright_yellow(), entry);
        }
        println!();
    }

    if !non_mods_and_directories.is_empty() {
        println!("ℹ️  {}", "Entries that are not java archives:".blue());
        for entry in non_mods_and_directories {
            println!("{} {}", "•".bright_blue(), entry);
        }
    }

    //      match api::query_modrinth_api("create") {
    //          Ok(query_result) => {
    //              match query_result {
    //                  QueryResult::ServerBad => println!("❌ That mod shouldn't really be on a server!"),
    //                  QueryResult::ServerOk => println!("✅ That mod can be on a server!"),
    //                  QueryResult::Unknown => println!("❔ Author didn't provide info about if mod works on server side."),
    //              }
    //          }
    //          Err(e) => {
    //              match e {
    //                  QueryError::Ureq(error) => println!("UREQ ERROR: {}", error.to_string()),
    //                  QueryError::Json(error) => println!("JSON ERROR: {}", error.to_string()),
    //                  QueryError::ServerSideEntryNotFound => println!("ERROR: Not found field \"server_side\" in response"),
    //              }
    //          }
    //      }
}
