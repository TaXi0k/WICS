use std::fs::{self, File};
use std::io::Read;
use std::path::{Path};
use mimetype_detector::{self};
use zip::ZipArchive;
use toml::Table;

pub enum Type {
    ClientMod,
    ServerMod,
    Directory,
    NotJAR,
}

pub fn check_file(input_path: &str, file_path: &str) -> String {
    let path = Path::new(input_path).join(file_path);

    // PROVIDED PATH IS A FILE
    if path.is_file() {

        // PROVIDED PATH IS A .jar
        if mimetype_detector::detect_file(&path).unwrap().is(mimetype_detector::APPLICATION_JAVA_ARCHIVE) {
            
            
            match File::open(&path) {   // Try to open .jar file
                Ok(file) => {   //File opened succesfully
                    match ZipArchive::new(file) {   // Try to open archive
                        Ok(archive) => {    // Archive opened succesfully
                            match check_modid(archive) {
                                Ok(modid) => modid,
                                Err(e) => e,
                            }
                        }
                        Err(e) => format!("Failed to open archive with error: {e}") // Failed to open archive
                    }
                }
                Err(e) => format!("Failed to open file with error: {e}")    // Failed to open file
            }
        }
        
        // PROVIDED PATH IS NOT A .jar
        else {
            format!("File {} is a file but not a .jar!!!!", file_path)
        }

    } 
    
    // PROVIDED PATH IS A DIRECTORY
    else if path.is_dir() {
        format!("Entry {} is a directory!", file_path)
    }
    
    // FALLBACK
    else {
        fs::read_to_string(&path).unwrap_or("File is not readable to string!".to_string())
    }
}


// Wrapps all mod loaders together, checks all 3 possible meta files
fn check_modid(archive: ZipArchive<File>) -> Result<String, String> {
    if let Some(modid) = check_neoforge_modid(archive) {
        Ok(modid)
    }
    else { Err("Failed to find modid in Forge, NeoForge and Fabric meta files.".to_string()) }
}

// Grabs modid of Fabric/Quilt mods
fn check_fabric_modid(mut archive: ZipArchive<File>) -> Option<String> {
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
                                //Err(format!("Failed to find \"id\" inside of \"fabric.mod.json\""))
                                None
                            }  
                        }
                    },
                    Err(e) => {     // Failed to parse content of "fabric.mod.json" as json
                        //Err(format!("Failed to parse contents of \"fabric.mod.json\" as json, with error: {e}"))
                        None
                    }
                }
            } else {    // Failed to read fabric.mod.json to string
                //Err("Failed to read \"fabric.mod.json\" to string.".to_string())
                None
            }
        }
        Err(e) => {  // Failed to open "fabric.mod.json"
            //Err(format!("Failed to open \"fabric.mod.json\" inside archive, with error: {e}"))
            None
        }
    }
    
}
// Grabs modid of NeoForge mods
fn check_neoforge_modid(mut archive: ZipArchive<File>) -> Option<String> {
    //check "modId" of /META-INF/neoforge.mods.toml
    match archive.by_name("META-INF/neoforge.mods.toml") {  // Try to open "META-INF/neoforge.mods.toml"
        Ok(mut file) => {   // Opened "META-INF/neoforge.mods.toml" succesfully
            let mut contents = String::new();   // Create buffer for reading contents of "META-INF/neoforge.mods.toml"
            if file.read_to_string(&mut contents).is_ok() {     // Reading "META-INF/neoforge.mods.toml" to string succeeded
                let toml_parsed = contents.parse::<Table>().unwrap();
                Some(toml_parsed["modId"].to_string())
            } else {    // Failed to read META-INF/neoforge.mods.toml to string
                //Err("Failed to read \"META-INF/neoforge.mods.toml\" to string.".to_string())
                None
            }
        }
        Err(e) => {  // Failed to open "META-INF/neoforge.mods.toml"
            //Err(format!("Failed to open \"META-INF/neoforge.mods.toml\" inside archive, with error: {e}"))
            None
        }
    }
}
//  // Grabs modid of Forge mods
//  fn check_forge_modid(archive: ZipArchive<File>) -> String {
//      //check "modId" of /META-INF/mods.toml
//  }
