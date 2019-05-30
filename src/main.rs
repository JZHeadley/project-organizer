use std::env;
use std::fs;

const BASE_PROJECTS_DIR:&str = "/home/projects/";
const CONFIG_DIR:&str = "/home/jzheadley/.projects/";

fn main() {
    let mut args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    args.drain(0..1); // remove the first element because its the program path which we don't care about
    
    if let Some((command, commandArgs)) = args.split_first() {
        handle_command(command, &Vec::from(commandArgs));
    }

}
fn handle_command(command: &String, commandArgs: &Vec<String>) {
    println!("{}", command);
    println!("{:?}", commandArgs);
    
    match command.as_ref() {
        "new" => {
            let directory_name: String = commandArgs.first().unwrap().to_owned();
            create_directory(&directory_name)
        }
        _ => println!("Command not found"),
    }
}
fn create_directory(directory_name: &String) {
    println!("{:?}", directory_name);
    fs::create_dir_all(String::from(BASE_PROJECTS_DIR) + directory_name);
}
