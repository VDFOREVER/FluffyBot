use std::collections::HashMap;
use std::fs;

pub fn read_history(file_path: &str) -> Result<HashMap<String, Vec<String>>, Box<dyn std::error::Error>> {
    if let Ok(contents) = fs::read_to_string(file_path) {
        let map: HashMap<String, Vec<String>> = serde_json::from_str(&contents)?;
        Ok(map)
    } else {
        Ok(HashMap::new())
    }
}

pub fn write_history(file_path: &str, history: &HashMap<String, Vec<String>>) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = std::path::Path::new(file_path).parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }
    let json = serde_json::to_string(&history)?;
    fs::write(file_path, json)?;
    Ok(())
}