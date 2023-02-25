use std::path::{Path, PathBuf};
use fs_extra::{self, dir::CopyOptions};

pub fn greet(){
    let message = r###"
    Rex CLI is a scaffolder that helps you quickly bootstrap a new Express project using JavaScript or TypeScript
    Read more about Rex at https://github.com/TheWisePigeon/rex#README 
    "###;
    println!("{}", message)
}

pub fn print_help_message(){
    println!("Visit https://github.com/TheWisePigeon/rex#README for help or reach out to me on Discord TheWisePigeon#2675")
}

fn verify_if_dir_exists(project_name: &String){
    let current_location = std::env::current_dir().unwrap().to_path_buf();
    let project_path = format!("{}\\{}", current_location.to_str().unwrap(), project_name);
    let project_dir = Path::new( &project_path );
    if project_dir.is_dir(){
        println!("A directory named {project_name} already exists in the current directory");
        return;
    }
}

fn get_project_path(project_name: &str) -> PathBuf {
    let current_location = std::env::current_dir().unwrap().to_path_buf();
    let project_path = format!("{}\\{}", current_location.to_str().unwrap(), project_name);
    PathBuf::from( &project_path )
}

fn copy_files(project_path: String, template_location: String){
    println!("{project_path}");
    let template_entries = std::fs::read_dir(template_location.strip_prefix(r#"\\?\"#).unwrap()).expect("something fucked");
    let mut items_paths: Vec<String> = vec![];
    for item in template_entries{
        items_paths.push(item.unwrap().path().to_str().unwrap().to_string());
    }
    fs_extra::copy_items(&items_paths, project_path, &CopyOptions::new()).expect("Error while initializing project");
}


// pub fn add_service(){
//D:\Personal projects\rex\target\debug\js-template
// }

pub fn init( argument: &str, project_name: Option<String> ){
    if argument!="js" && argument!="ts" {
        println!("Invalid argument \"{argument}\". Only `ts` and `js` are valid arguments for the init command");
        return;
    }
    let rex_binary_location = std::env::current_exe().unwrap();
    let rex_home = rex_binary_location.parent().unwrap();
    if let Some(home) = rex_home.to_str(){
        let template_location = format!("{home}\\templates\\{argument}-template");
        if let Some(project_name) = project_name{
            verify_if_dir_exists(&project_name);
            std::fs::create_dir(get_project_path(&project_name.clone())).expect("Failed to create project directory");
            let project_path = get_project_path(project_name.as_str());
            copy_files( String::from(project_path.to_str().unwrap()), template_location);
        }else{
            let mut project_name: String = String::from("");
            println!("What is your project name? Leave blank to use current directory(Works for empty directories only)");
            std::io::stdin().read_line(&mut project_name).expect("Failed to read project name");
            if project_name.trim()==""{
                println!("Nigga");
                return;
            }
            verify_if_dir_exists(&String::from(project_name.trim_end()));
    
        }
    }
}