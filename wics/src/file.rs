use std::fs::{self, File};
use std::io::Read;
use std::path::{Path};
use mimetype_detector::{self};
use zip::ZipArchive;
use toml::Value;
use crate::api::{QueryError, QueryResult, query_modrinth_api};
use crate::tools::{log, LogType::*};

pub enum CheckResult {
    ServerOk,
    ServerBad,
    Unknown,
    Directory,
    NonJAR,
}

pub fn check_file(input_path: &str, file_path: &str) -> Option<CheckResult> {
    let path = Path::new(input_path).join(file_path);

    // PROVIDED PATH IS A FILE
    if path.is_file() {

        // PROVIDED PATH IS A .jar
        //if mimetype_detector::detect_file(&path).unwrap().is(mimetype_detector::APPLICATION_JAVA_ARCHIVE) {
        match path.extension() {
            Some(extension) => {
                if extension == "jar" {  //check if file is a java archive
                    match File::open(&path) {   // Try to open .jar file
                        Ok(file) => {   //File opened succesfully
                            match ZipArchive::new(file) {   // Try to open archive
                                Ok(mut archive) => {    // Archive opened succesfully
                                    match check_modid(&mut archive) {
                                        Ok(modid) => {
                                        
                                        
                                            //CALL API AND ALL THAT THINGYS
                                            match query_modrinth_api(&modid) {
                                                Ok(query_result) => {
                                                    match query_result {
                                                        QueryResult::ServerOk => {
                                                            log(&format!("Mod {file_path} (id: {modid}) IS supported on server side."), Info);
                                                            Some(CheckResult::ServerOk)
                                                        },
                                                        QueryResult::ServerBad => {
                                                            log(&format!("Mod {file_path} (id: {modid}) IS NOT supported on server side."), Info);
                                                            Some(CheckResult::ServerBad)
                                                        },
                                                        QueryResult::Unknown => {
                                                            log(&format!("Author of mod {file_path} (id: {modid}) didn't provide information about support of mod on server side (\"server_side\": \"unknown\")."), Info);
                                                            Some(CheckResult::Unknown)
                                                        },
                                                    }
                                                },
                                                Err(error) => {
                                                    match error {
                                                        QueryError::Ureq(e) => {
                                                            log(&format!("Failed to call api for mod {file_path} with error: {e}"), Error);
                                                            None
                                                        },
                                                        QueryError::Json(e) => {
                                                            log(&format!("Failed to parse response for mod {file_path} as json with error: {e}"), Error);
                                                            None
                                                        },
                                                        QueryError::ServerSideEntryNotFound => {
                                                            log(&format!("Failed to find entry \"server_side\" in parsed api response for mod {file_path}"), Error);
                                                            None
                                                        }
                                                    }
                                                },
                                            }
                                        
                                        
                                        },
                                        Err(e) => {
                                            log(&format!("Failed to get modid of mod {file_path} with following error: {e}"), Error);
                                            None
                                        },
                                    }
                                }
                                Err(e) => {     // Failed to open archive
                                    log(&format!("Failed to open file {file_path} as ZipArchive with error: {e}"), Error);
                                    None
                                }
                            }
                        }
                        Err(e) => {     // Failed to open file
                            log(&format!("Failed to open file {file_path} with error: {e}"), Error);
                            None
                        }
                    }
                }
                else {  // PROVIDED PATH IS NOT A .jar
                    log(&format!("File {file_path} is not a java archive"), Info);
                    Some(CheckResult::NonJAR)
                }
            },
            None => {
                log(&format!("File {file_path} is not a java archive (doesn't have extension)"), Info);
                Some(CheckResult::NonJAR)
            },
        }

    } 
    
    // PROVIDED PATH IS A DIRECTORY
    else if path.is_dir() {
        log(&format!("File {file_path} is a directory."), Info);
        Some(CheckResult::Directory)
    }
    
    // FALLBACK
    else {
        log(&format!("File {file_path} is not readable to string. (that's fallback which you shouldn't really see)"), Warning);
        None
    }
}


// Wrapps all mod loaders together, checks all 3 possible meta files
fn check_modid(archive: &mut ZipArchive<File>) -> Result<String, String> {
    if let Some(modid) = check_fabric_modid(archive) {
        Ok(modid)
    }
    else if let Some(modid) = check_neoforge_modid(archive) {
        Ok(modid)
    }
    else if let Some(modid) = check_forge_modid(archive) {
        Ok(modid)
    }
    else {
        Err("Failed to find modid in Forge, NeoForge and Fabric meta files. (run wics with -log flag to see (a lot) more details)".to_string())
    }
}

// Grabs modid of Fabric/Quilt mods
fn check_fabric_modid(archive: &mut ZipArchive<File>) -> Option<String> {
    //check "id" of fabric.mod.json
    match archive.by_name("fabric.mod.json") {  // Try to open "fabric.mod.json"
        Ok(mut file) => {   // Opened "fabric.mod.json" succesfully
            let mut contents = String::new();   // Create buffer for reading contents of "fabric.mod.json"
            if file.read_to_string(&mut contents).is_ok() {     // Reading "fabric.mod.json" to string succeeded
                match json::parse(&contents) {      // Try to parse content of "fabric.mod.json" as json
                    Ok(json_parsed) => {        // Parsed as json succesfully
                        match json_parsed["id"].as_str() {      // Try to get value of "id" from that json
                            Some(modid) => Some(modid.to_string()),   // Found "id" and returned it
                            None => {   // Not found "id" inside of json
                                log("Failed to find \"id\" inside of \"fabric.mod.json\"", Warning);
                                None
                            }  
                        }
                    },
                    Err(e) => {     // Failed to parse content of "fabric.mod.json" as json
                        log(&format!("Failed to parse contents of \"fabric.mod.json\" as json, with error: {e}"), Warning);
                        None
                    }
                }
            } else {    // Failed to read fabric.mod.json to string
                log("Failed to read \"fabric.mod.json\" to string.", Warning);
                None
            }
        }
        Err(e) => {  // Failed to open "fabric.mod.json"
            log(&format!("Failed to open \"fabric.mod.json\" inside archive, with error: {e}"), Warning);
            None
        }
    }
    
}
// Grabs modid of NeoForge mods
fn check_neoforge_modid(archive: &mut ZipArchive<File>) -> Option<String> {
    //check "modId" of /META-INF/neoforge.mods.toml
    match archive.by_name("META-INF/neoforge.mods.toml") {  // Try to open "META-INF/neoforge.mods.toml"
        Ok(mut file) => {   // Opened "META-INF/neoforge.mods.toml" succesfully
            let mut contents = String::new();   // Create buffer for reading contents of "META-INF/neoforge.mods.toml"
            if file.read_to_string(&mut contents).is_ok() {     // Reading "META-INF/neoforge.mods.toml" to string succeeded
                match extract_first_modid_from_toml(&contents) {
                    Some(mod_id) => Some(mod_id) ,
                    None => {
                        log("Failed to find modId in neoforge.mods.toml", Warning);
                        None
                    }
                }             
            } else {    // Failed to read META-INF/neoforge.mods.toml to string
                log("Failed to read \"META-INF/neoforge.mods.toml\" to string", Warning);
                None
            }
        }
        Err(e) => {  // Failed to open "META-INF/neoforge.mods.toml"
            log(&format!("Failed to open \"META-INF/neoforge.mods.toml\" inside archive, with error: {e}"), Warning);
            None
        }
    }
}
// Grabs modid of Forge mods
fn check_forge_modid(archive: &mut ZipArchive<File>) -> Option<String> {
    //check "modId" of /META-INF/neoforge.mods.toml
    match archive.by_name("META-INF/mods.toml") {  // Try to open "META-INF/neoforge.mods.toml"
        Ok(mut file) => {   // Opened "META-INF/neoforge.mods.toml" succesfully
            let mut contents = String::new();   // Create buffer for reading contents of "META-INF/neoforge.mods.toml"
            if file.read_to_string(&mut contents).is_ok() {     // Reading "META-INF/neoforge.mods.toml" to string succeeded
                match extract_first_modid_from_toml(&contents) {
                    Some(mod_id) => Some(mod_id) ,
                    None => {
                        //Err(("Failed to find modId in forge.mods.toml").to_string())
                        log("Failed to find modId in mods.toml", Warning);
                        None
                    }
                }             
            } else {    // Failed to read META-INF/neoforge.mods.toml to string
                log("Failed to read \"META-INF/mods.toml\" to string", Warning);
                None
            }
        }
        Err(e) => {  // Failed to open "META-INF/neoforge.mods.toml"
            log(&format!("Failed to open \"META-INF/mods.toml\" inside archive, with error: {e}"), Warning);
            None
        }
    }
}

fn extract_first_modid_from_toml(toml_content: &str) -> Option<String> {
    let parsed_data: Value = toml::from_str(toml_content).ok()?;    // Parse provided string to toml
    let mod_id = parsed_data    
        .get("mods")?      // Get the "mods" key since in (neoforge.)mods.toml it modId is inside of [[mods]]
        .get(0)?           // Get 1st element in array since it turns out that's what you have to do
        .get("modId")?     // Get the "modId" key 
        .as_str()?;                     // Extract it as &str

    Some(mod_id.to_string())            // Return the modid
}