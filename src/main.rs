mod settings;
mod projects;

use std::env;
use std::fs::{self};
use settings::Settings;
use projects::{Project, Projects};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    args.drain(0..1); // remove the first element because its the program path which we don't care about
    let user_home = env::var("HOME").expect("No environment variable $HOME");

    let config: Settings = Settings::read_config(user_home.clone());
    let (config, mut projects): (Settings, Vec<Project>) = Projects::read_projects(config);
    println!("{:#?}", config);
    println!("{:#?}", projects);
    if let Some((command, command_args)) = args.split_first() {
        handle_command(command, Vec::from(command_args), &config, projects);
    }
}

fn handle_command(
    command: &String,
    command_args: Vec<String>,
    config: &Settings,
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
            Projects::write_projects(projects, config);
        }
        "tag" => {
            let project_name: String = command_args.first().unwrap().to_owned();
            let new_tags: Vec<String> = Vec::from(&command_args[1..]);
            // search for project and add new tags
        }
        _ => println!("Command not found"),
    }
}

fn create_directory(directory_name: &String, config: &Settings) {
    // println!("{:?}", directory_name);
    fs::create_dir_all(String::from(config.base_projects_dir.clone()) + directory_name)
        .expect("Couldn't create the project directory.");
}
