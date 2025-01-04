use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    pub last_visited_path: Option<String>,
}

pub fn load_state() -> State {
    let state_file_paths = [home::home_dir()
        .unwrap()
        .as_path()
        .join(".config/rustify/config/state.toml")];

    let mut content: String = "".to_owned();

    for state_file_path in state_file_paths {
        let result: Result<String, std::io::Error> = fs::read_to_string(state_file_path);

        if let Ok(file_content) = result {
            content = file_content;
            break;
        }
    }

    let state_toml: State = toml::from_str(&content).unwrap_or_else(|_| {
        eprintln!("UNexpected Error: Could not read the state file. Using default values.");
        State {
            last_visited_path: None,
        }
    });

    state_toml
}
pub fn save_state(state: State) -> Result<(), String> {
    let state_file_path = home::home_dir()
        .unwrap()
        .as_path()
        .join(".config/rustify/config/state.toml");

    toml::to_string(&state).map_err(|e| e.to_string())
        .and_then(|serialized| fs::write(state_file_path, serialized).map_err(|e| e.to_string()))
}