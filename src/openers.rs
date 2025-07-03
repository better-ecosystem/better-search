use std::collections::HashMap;
use std::fs;
use toml::Value;
use std::path::PathBuf;
use shellexpand;

pub struct OpenersConfig {
    pub openers: HashMap<String, String>,
    pub app_dirs: Vec<String>,
}

pub fn get_openers() -> OpenersConfig {
    let mut openers_map = HashMap::new();
    let mut app_dirs_vec = Vec::new();

    let path = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"))
        .join("search/openers.toml");

    if let Ok(contents) = fs::read_to_string(&path) {
        if let Ok(parsed) = contents.parse::<Value>() {
            if let Some(table) = parsed.get("openers").and_then(|v| v.as_table()) {
                for (k, v) in table {
                    if let Some(cmd) = v.as_str() {
                        openers_map.insert(k.clone(), cmd.to_string());
                    }
                }
            }

            if let Some(array) = parsed.get("config")
                           .and_then(|v| v.get("app_dirs"))
                           .and_then(|v| v.as_array()) {
            for item in array {
                if let Some(path_str) = item.as_str() {
                    let expanded = shellexpand::tilde(path_str).to_string();
                    app_dirs_vec.push(expanded);
                }
            }
        }

        }
    }

    OpenersConfig {
        openers: openers_map,
        app_dirs: app_dirs_vec,
    }
}

