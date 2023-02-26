use fs_extra::{self, dir::CopyOptions};
use std::{path::{Path, PathBuf}, fs};

pub fn greet() {
    let message = r###"
    Rex CLI is a scaffolder that helps you quickly bootstrap a new Express project using JavaScript or TypeScript
    Read more about Rex at https://github.com/TheWisePigeon/rex#README 
    "###;
    println!("{}", message)
}

pub fn print_help_message() {
    println!("Visit https://github.com/TheWisePigeon/rex#README for help or reach out to me on Discord TheWisePigeon#2675")
}

fn verify_if_dir_exists(project_name: &String) {
    let current_location = std::env::current_dir().unwrap().to_path_buf();
    let project_path = format!("{}\\{}", current_location.to_str().unwrap(), project_name);
    let project_dir = Path::new(&project_path);
    if project_dir.is_dir() {
        println!("A directory named {project_name} already exists in the current directory");
        return;
    }
}

fn get_project_path(project_name: &str) -> PathBuf {
    let current_location = std::env::current_dir().unwrap().to_path_buf();
    let project_path = format!("{}\\{}", current_location.to_str().unwrap(), project_name);
    PathBuf::from(&project_path)
}

fn copy_files(project_path: String, template_location: String) {
    let template_entries = std::fs::read_dir(template_location)
        .expect("something fucked");
    let mut items_paths: Vec<String> = vec![];
    for item in template_entries {
        items_paths.push(item.unwrap().path().to_str().unwrap().to_string());
    }
    fs_extra::copy_items(&items_paths, project_path, &CopyOptions::new())
        .expect("Error while initializing project");
    println!(r#"
        Rex project initialized.
        Now cd into your project and run npm install
        Rex is at https://github.com/TheWisePigeon/rex
    "#);
}

// pub fn add_service(){
//D:\Personal projects\rex\target\debug\js-template
// }

pub fn init(argument: &str, project_name: Option<String>) {
    if argument != "js" && argument != "ts" {
        println!("Invalid argument \"{argument}\". Only `ts` and `js` are valid arguments for the init command");
        return;
    }
    let rex_binary_location = std::env::current_exe().unwrap();
    let rex_home = rex_binary_location.parent().unwrap();
    if let Some(home) = rex_home.to_str() {
        let template_location = format!("{home}\\templates\\{argument}-template");
        if let Some(project_name) = project_name {
            verify_if_dir_exists(&project_name);
            std::fs::create_dir(get_project_path(&project_name.clone()))
                .expect("Failed to create project directory");
            let project_path = get_project_path(project_name.as_str());
            copy_files(
                String::from(project_path.to_str().unwrap()),
                template_location
            );
            return;
        } else {
            let mut project_name: String = String::from("");
            println!("What is your project name? Leave blank to use current directory(Works for empty directories only)");
            std::io::stdin()
                .read_line(&mut project_name)
                .expect("Failed to read project name");
            if project_name.trim() == "" {
                if let Ok(mut dir) = std::fs::read_dir(
                    std::env::current_dir().expect("Error while reading current directory"),
                ) {
                    if !dir.next().is_none() {
                        println!("Directory is not empty. Can't initialize Rex project");
                        return;
                    }
                    copy_files(String::from(""), template_location);
                    return;
                }
            }
            verify_if_dir_exists(&String::from(project_name.trim_end()));
            std::fs::create_dir(get_project_path(&project_name.trim_end().clone()))
                .expect("Failed to create project directory");
            let project_path = get_project_path(project_name.as_str().trim_end());
            copy_files(
                String::from(
                    project_path
                        .to_str()
                        .expect("Error while initializing project"),
                ),
                template_location
            );
            return;
        }
    }
}

pub fn add_service(name: String, extension: &str) {
    let error = "Error while adding service";
    let current_directory = std::env::current_dir()
        .expect("Error while reading current directory");
    let current_directory = current_directory
        .to_str()
        .expect("Error while reading current directory");
    let service_path = format!("{current_directory}\\src\\services\\{name}.{extension}");
    if Path::new(&service_path).is_dir(){
        println!("This service already exists apparently");
        return;
    }
    let route_path = format!("{current_directory}\\src\\routes\\{name}");
    let route_index_path = format!("{route_path}\\index.{extension}");
    fs::File::create(service_path).expect(error);
    fs::create_dir(route_path).expect(error);
    fs::File::create(route_index_path).expect(error);
    println!("Added service {name}");
    return;
}
