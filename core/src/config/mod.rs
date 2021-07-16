use keyring::KeyringError;
use std::collections::HashMap;
use std::path::Path;
use std::io::{Write, Read};
use serde::{Serialize, Deserialize};
use std::fs::DirEntry;

const APP_NAME : &str = "kaf9s";

pub fn get_secret_from_keyring(input : &str) -> Result<String, KeyringError> {
    let key = get_key(&input);
    let key_store = keyring::Keyring::new(&key, "");
    key_store.get_password()
}

pub fn get_key(input : &str) -> String {
    format!("{}/{}", APP_NAME, input)
}

pub fn set_secret_in_keyring(input : &str, val : &str) -> Result<(), KeyringError> {
    let key = get_key(&input);
    let key_store = keyring::Keyring::new(&key, "");
    key_store.set_password(val)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Context {
    pub name : String,
    pub cluster : String,
    pub user : String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Cluster {
    pub name : String,
    pub data : HashMap<String, String>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub name : String,
    pub data : HashMap<String, String>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ContextFile {
    pub contexts : Vec<Context>,
    pub clusters : Vec<Cluster>,
    pub users : Vec<User>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(default)]
pub struct Config {
    pub refresh_rate : u32, // in milliseconds
    pub current_context : Option<String>
}

impl Default for Config {
    fn default() -> Self {
        Self {
            refresh_rate: 2000,
            current_context: None
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConfigManager {
    pub contexts : HashMap<String, Context>,
    pub clusters : HashMap<String, Cluster>,
    pub users : HashMap<String, User>,
    pub conf : Config
}

impl ConfigManager{
    pub fn load() -> ConfigManager {
        let config_dir = dirs::home_dir().expect("Unable to get user home directory").join(".config/kaf9s");
        let mut conf = Config::default();

        if !config_dir.exists() {
            std::fs::create_dir(config_dir.as_path()).unwrap();
        }

        // config.yaml
        let config_file_path = config_dir.join("config.yaml");
        if !config_file_path.exists() {
            let serialised = serde_yaml::to_vec(&conf).expect("Unable to convert default Config to YAML");
            let mut file = std::fs::File::create(config_file_path.as_path()).expect("Unable to create config.yaml");
            file.write_all(&serialised);
        } else {
            let mut file = std::fs::File::open(config_file_path.as_path()).expect("Unable to read config.yaml");
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer);
            conf = serde_yaml::from_slice(&buffer).expect("Unable to deserialise config.yaml");
        }

        // config_*.yaml
        let dir = std::fs::read_dir(config_dir.as_path()).expect("Unable to read $HOME/.config/kaf9s");
        let context_files : Vec<DirEntry> = dir
            .filter(|file| {
              return match file {
                  Ok(file) => {
                      file.file_name().to_str().expect("Unable to convert to str").contains("config_")
                  },
                  Err(_) => false
              }
            })
            .map(|file| {
                file.expect("No DirEntry, something went quite wrong")
            })
            .collect();

        // Set up ConfigManager
        let mut cm = Self {
            contexts: Default::default(),
            clusters: Default::default(),
            users: Default::default(),
            conf
        };

        for context_file in context_files {
            let mut file = std::fs::File::open(context_file.path().as_path()).expect(format!("Unable to read {}", context_file.file_name().to_str().unwrap()).as_str());
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer);
            let context_file : ContextFile = serde_yaml::from_slice(&buffer).expect(format!("Unable to deserialise {}", context_file.file_name().to_str().unwrap()).as_str());

            for cluster in &context_file.clusters {
                if let Some(_) = cm.clusters.insert(cluster.name.clone(), cluster.clone()) {
                    println!("Duplicate Cluster entry discovered, overriding existing entry '{}'", &cluster.name);
                }
            }

            for context in &context_file.contexts {
                if let Some(_) = cm.contexts.insert(context.name.clone(), context.clone()) {
                    println!("Duplicate Context entry discovered, overriding existing entry '{}'", &context.name);
                }
            }

            for user in &context_file.users {
                if let Some(_) = cm.users.insert(user.name.clone(), user.clone()) {
                    println!("Duplicate User entry discovered, overriding existing entry '{}'", &user.name);
                }
            }
        }

        cm
    }
}