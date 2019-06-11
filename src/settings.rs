use config;

use std::collections::HashMap;
use std::fs::{self,File};
use std::path::Path;


#[derive(Debug)]
pub struct Settings {
    pub base_projects_dir: String,
    pub project_config_path: String,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            base_projects_dir: String::from("/home/projects"),
            project_config_path: String::from("/home/projects/projects.toml"),
        }
    }
}
impl Settings{
    pub fn read_config(user_home: String) -> Settings {
        let config_dir_str: String = String::from(user_home + "/.projects");
        let config_path_str = String::from(config_dir_str.clone() + "/config.toml");
        let config_path: &Path = Path::new(&config_path_str);
        let mut config: Settings = Settings::default();
        if !config_path.exists() {
            println!("Creating bare config file");
            fs::create_dir_all(config_dir_str).expect("Couldn't create the config directory.");
            File::create(&config_path).expect("Error creating config file");
        }
        let mut settings = config::Config::default();
        settings
            .merge(config::File::with_name(&config_path_str))
            .unwrap();
        let settings_map: HashMap<String, String> = settings
            .try_into::<HashMap<String, String>>()
            .expect("couldn't read settings");
        // println!("settings path of :{} has settings: {:#?}", config_path_str,settings_map);
        if settings_map.contains_key("base_projects_dir") {
            config.base_projects_dir = settings_map.get("base_projects_dir").unwrap().clone();
        }
        if settings_map.contains_key("project_config_path") {
            config.project_config_path = settings_map.get("project_config_path").unwrap().clone();
        }
        return config;
    }
}