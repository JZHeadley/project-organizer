extern crate config;

use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;

use std::collections::HashMap;


#[derive(Debug)]
struct Config {
    base_projects_dir: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            base_projects_dir: String::from("/home/projects"),
        }
    }
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    args.drain(0..1); // remove the first element because its the program path which we don't care about
    let user_home = env::var("HOME").expect("No environment variable $HOME");

    let config: Config = read_config(user_home);
    println!("{:?}",config);
    if let Some((command, command_args)) = args.split_first() {
        handle_command(command, &Vec::from(command_args), &config);
    }

}

fn read_config(user_home: String) -> Config {
    let config_dir_str: String = String::from(user_home + "/.projects");
    let config_path_str = String::from(config_dir_str.clone() + "/config");
    let config_path: &Path = Path::new(&config_path_str);
    let mut config_file: File;
    let mut config: Config = Config::default();
    if !config_path.exists() {
        println!("Creating bare config file");
        fs::create_dir_all(config_dir_str).expect("Couldn't create the config directory.");
        config_file = File::create(&config_path).expect("Error creating config file");
    } else {
        config_file = File::open(&config_path).expect("error opening config file");
    }
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Settings")).unwrap();
    let settings_map: HashMap<String, String> = settings
        .try_into::<HashMap<String, String>>()
        .expect("couldn't read settings");
    if settings_map.contains_key("base_projects_dir") {
        config.base_projects_dir = settings_map.get("base_projects_dir").unwrap().clone();
    }
    return config;
}

fn handle_command(command: &String, command_args: &Vec<String>, config: &Config) {
    // println!("{}", command);
    // println!("{:?}", command_args);

    match command.as_ref() {
        "new" => {
            let directory_name: String = command_args.first().unwrap().to_owned();
            create_directory(&directory_name, &config)
        }
        _ => println!("Command not found"),
    }
}

fn create_directory(directory_name: &String, config: &Config) {
    // println!("{:?}", directory_name);
    fs::create_dir_all(String::from(config.base_projects_dir.clone()) + directory_name)
        .expect("Couldn't create the project directory.");
}
