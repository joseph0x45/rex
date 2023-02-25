use fs_extra::{self, dir::CopyOptions};
use std::path::{Path, PathBuf};

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
    let template_entries = std::fs::read_dir(template_location.strip_prefix(r#"\\?\"#).unwrap())
        .expect("something fucked");
    let mut items_paths: Vec<String> = vec![];
    for item in template_entries {
        items_paths.push(item.unwrap().path().to_str().unwrap().to_string());
    }
    fs_extra::copy_items(&items_paths, project_path, &CopyOptions::new())
        .expect("Error while initializing project");
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
                template_location,
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
                template_location,
            );
            return;
        }
    }
}

pub fn add_service() {
    let current_dir = std::env::current_dir().expect("Error while reading current directory");
    let rex_conf_path = format!(
        "{}\\rex.conf",
        current_dir
            .to_str()
            .expect("Error while reading current directory")
    );
    if !Path::new(&rex_conf_path).is_dir() {
        println!(
            "It seems like you are not in a rex generated project, the rex.conf file is missing"
        );
        return;
    }
    let rex_conf_content = std::fs::read_to_string(rex_conf_path).expect("Failed to read rex.conf");
    let rex_conf: Vec<&str> = rex_conf_content.split("\n").collect();
    let mut rex_template: &str = "";
    for config in rex_conf {
        let splitted_line: Vec<&str> = config.split("=").collect();
        if splitted_line.len() < 3 {
            println!("Error while reading rex.conf. A key is malformed {config}");
            return;
        }
        if splitted_line[0] == "TEMPLATE" {
            rex_template = splitted_line[2];
            break;
        }
    }
    if rex_template == "" {
        println!("Failed to read template type from rex.conf");
        return;
    }
}
