extern crate config;

use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::path::Path;


use serde::{Deserialize, Serialize};

#[derive(Debug)]
struct Config {
    base_projects_dir: String,
    project_config_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    project_name: String,
    tags: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Projects {
    projects: Vec<Project>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            base_projects_dir: String::from("/home/projects"),
            project_config_path: String::from("/home/projects/projects.toml"),
        }
    }
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    args.drain(0..1); // remove the first element because its the program path which we don't care about
    let user_home = env::var("HOME").expect("No environment variable $HOME");

    let config: Config = read_config(user_home.clone());
    let (config, mut projects): (Config, Vec<Project>) = read_projects(config);
    println!("{:#?}", config);
    println!("{:#?}", projects);
    if let Some((command, command_args)) = args.split_first() {
        handle_command(command, Vec::from(command_args), &config, projects);
    }
}

fn write_projects(projects:Vec<Project>, config: &Config){
    let projects_json :String = serde_json::to_string(&Projects{projects}).expect("Couldn't convert projects to json");
    fs::write(config.project_config_path.clone(),projects_json).expect("Couldn't write to projects file");
}

fn handle_command(
    command: &String,
    command_args: Vec<String>,
    config: &Config,
    projects_in: Vec<Project>,
) {
    // println!("{}", command);
    // println!("{:?}", command_args);
    let mut projects = projects_in;
    match command.as_ref() {
        "new" => {
            let directory_name: String = command_args.first().unwrap().to_owned();
            create_directory(&directory_name, &config);
            projects.push(Project {
                project_name: directory_name,
                tags: Vec::from(&command_args[1..]),
            });
    // println!("{:#?}", projects);
            write_projects(projects,config);
        }
        "tag" => {
            let project_name: String = command_args.first().unwrap().to_owned();
            let new_tags: Vec<String> = Vec::from(&command_args[1..]);
            // search for project and add new tags
        }
        _ => println!("Command not found"),
    }
}

fn create_directory(directory_name: &String, config: &Config) {
    // println!("{:?}", directory_name);
    fs::create_dir_all(String::from(config.base_projects_dir.clone()) + directory_name)
        .expect("Couldn't create the project directory.");
}

fn read_projects(config: Config) -> (Config, Vec<Project>) {
    let projects_path = config.project_config_path.clone();
    let projects_json_str: String = fs::read_to_string(projects_path).expect("File not found");
    // println!("{}", projects_json_str);
    let projects: Vec<Project> = serde_json::from_str(&projects_json_str)
        .unwrap_or(Projects {
            projects: Vec::new(),
        })
        .projects;
    (config, projects)
}

fn read_config(user_home: String) -> Config {
    let config_dir_str: String = String::from(user_home + "/.projects");
    let config_path_str = String::from(config_dir_str.clone() + "/config");
    let config_path: &Path = Path::new(&config_path_str);
    let mut config: Config = Config::default();
    if !config_path.exists() {
        println!("Creating bare config file");
        fs::create_dir_all(config_dir_str).expect("Couldn't create the config directory.");
        File::create(&config_path).expect("Error creating config file");
    }
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Settings")).unwrap();
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
