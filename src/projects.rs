use crate::settings::Settings;
use serde::{Deserialize, Serialize};
use std::fs::{self,File};


#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub project_name: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Projects {
    pub projects: Vec<Project>,
}

impl Projects{

    pub fn write_projects(projects: Vec<Project>, config: &Settings) {
        let projects_json: String =
            serde_json::to_string(&Projects { projects }).expect("Couldn't convert projects to json");
        fs::write(config.project_config_path.clone(), projects_json)
            .expect("Couldn't write to projects file");
    }

    pub fn read_projects(config: Settings) -> (Settings, Vec<Project>) {
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

}