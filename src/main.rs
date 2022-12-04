use std;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Config {
    template: String,
    version: String
}


fn main() {

    
    let action: Option<String> = std::env::args().nth(1);
    let arg1: Option<String> = std::env::args().nth(2);
    let arg2: Option<String> = std::env::args().nth(3);
    match action {
        Some(action)=>{
            match action.as_str() {
                "init"=>{
                    println!("
                    ██████╗░███████╗██╗░░██╗  ░█████╗░██╗░░░░░██╗
                    ██╔══██╗██╔════╝╚██╗██╔╝  ██╔══██╗██║░░░░░██║
                    ██████╔╝█████╗░░░╚███╔╝░  ██║░░╚═╝██║░░░░░██║
                    ██╔══██╗██╔══╝░░░██╔██╗░  ██║░░██╗██║░░░░░██║
                    ██║░░██║███████╗██╔╝╚██╗  ╚█████╔╝███████╗██║
                    ╚═╝░░╚═╝╚══════╝╚═╝░░╚═╝  ░╚════╝░╚══════╝╚═╝");
                    match arg1 {
                        Some(arg1)=>{
                            match arg1.as_str() {
                                "js" | "javascript"=>{
                                    println!("Bootstrapping JavaScript template");
                                    scaffolder(arg2, arg1);
                                    return
                                },
                                "ts" | "typescript"=>{
                                    println!("Bootstrapping TypeScript template");
                                    scaffolder(arg2, arg1);
                                    return
                                },
                                _=>{
                                    println!("Unrecognized template. Only typescript and javascript arg1s are supported by Rex for now.");
                                    return
                                }
                            }
                        },
                        None=>{
                            println!(" Please provide a template by running `rex init ts` or `rex init js` ");
                            return
                        }
                    }
                },
                //User can add a new service by using the add action
                "add"=>{
                    match arg1 {
                        Some(arg1)=>{
                            match arg1.as_str() {
                                "service"=>{
                                    add_service(arg2)
                                },
                                _=>{
                                    println!("Unrecognized parameter. Only `rex add service` is supported.");
                                    return
                                }
                            }
                        },
                        None=>{
                            println!("Invalid usage of `rex add`.\nRun `rex add service <service_name>` to add a new service");
                            return
                        }
                    }
                },
                "help"=>{
                    println!("
                        Welcome to the Rex documentation \n Please refeer to https://github.com/TheWisePigeon/rex#readme for the full extended documentation
                    ");
                    return
                },
                _=>{
                    println!("Unrecognized command");
                    std::process::exit(0)
                }
            }
        },
        None=>{
            println!("Run `rex init js` or `rex init ts` to quickly scaffold a new project");
            return
        }
    }
}


fn scaffolder( arg2: Option<String>, mut arg1: String ){
    //arg1s url match this string
    //https://github.com/TheWisePigeon/rex-<arg1_name>-arg1.git https://github.com/TheWisePigeon/rex-js-arg1.git
    //Check if the project name has been provided
    //Ask for it if not
    match arg1.as_str() {
        "js" | "javascript" | "JavaScript" =>{
            arg1 = String::from("js");
        },
        "ts" | "typescript" | "TypeScript" =>{
            arg1 = String::from("ts")
        },
        _=>{
            return;
        }
    }
    match arg2 {
        Some(arg2)=>{
            let root_folder: String = std::env::current_dir()
                .unwrap()
                .as_os_str()
                .to_str()
                .unwrap()
                .into();
            let project_folder: String = String::from(std::format!("{}\\{project}", root_folder, project=arg2));
            if std::path::Path::new(&project_folder).is_dir(){
                println!("A folder with the same name already exists");
                return
            }
            //Clone the content of the corresponding arg1 into the new folder
            std::process::Command::new("git")
                .arg("clone")
                .arg(std::format!("https://github.com/TheWisePigeon/rex-{arg1}-template.git"))
                .arg(std::format!("{arg2}"))
                .status()
                .expect("Something went wrong while setting up project");
            //Change current working directory into newly created project folder
            std::env::set_current_dir(&std::path::Path::new(&project_folder)).unwrap();
            //Delete the default remote origin branch
            std::process::Command::new(std::format!("git"))
                .arg("remote")
                .arg("remove")
                .arg("origin")
                .status()
                .expect("Something went wrong while setting up project. Please head over to https://github.com/TheWisePigeon/rex/issues to report the issue");

        },
        None=>{
            println!("Please provide a project name.\nHead over to https://github.com/TheWisePigeon/rex#readme to learn more about rex");
            return;
        }
    }
}


/// This function creates a new service by
/// - Creating a new file named after the service name in the services folder
/// - Adding a corresponding folder in the routes folder
fn add_service(service: Option<String>) {
    match service {
        Some(service)=>{
            //Check if user is in a rex generated project
            let root_folder: String = std::env::current_dir()
                .unwrap()
                .as_os_str()
                .to_str()
                .unwrap()
                .into();
            let rex_conf_json_path = std::format!("{root_folder}\\rex.conf.json");
            if !std::path::Path::new(&rex_conf_json_path).is_file() {
                println!("Make sure you are at the root of a rex generated project, and try again. Run `rex help` for the documentation");
                return;
            }
            //Add new file in services folder and add corresponding folder in routes folder
            let rex_conf_json_content = std::fs::read_to_string(std::path::Path::new(&rex_conf_json_path));
            match rex_conf_json_content {
                Ok(value)=>{
                    //Get the rex template in use by current project and determine service extension
                    let parsed_conf: Config = serde_json::from_str(&value).unwrap();
                    let extension: String;
                    match parsed_conf.template.as_str() {
                        "typescript"=>{
                            extension=String::from("ts");
                        },
                        "javascript"=>{
                            extension=String::from("js");
                        }
                        _=>{
                            println!("Unrecognized template in rex.conf.json.\nDid you edit this file? If not, please head over to https://github.com/TheWisePigeon/rex/issues to report the issue");
                            return
                        }
                    }
                    //Create file in services folder
                    std::fs::File::create(std::format!("{root_folder}\\src\\services\\{service}.{extension}"))
                        .expect("Error while creating service. \nPlease head over to https://github.com/TheWisePigeon/rex/issues to report the issue");
                    //Create folder in routes folder
                    std::fs::create_dir(std::format!("{root_folder}\\src\\routes\\{service}"))
                        .expect("Error while creating service.  \nPlease head over to https://github.com/TheWisePigeon/rex/issues to report the issue");
                    //Create index file in corresponding routes folder
                    std::fs::File::create(std::format!("{root_folder}\\src\\routes\\{service}\\index.{extension}"))
                        .expect("Error while creating service.  \nPlease head over to https://github.com/TheWisePigeon/rex/issues to report the issue");
                    //Exit program
                    println!("{} service added", service);
                    return
                },
                Err(_)=>{
                    println!("Something went wrong while creating service.\nPlease head over to https://github.com/TheWisePigeon/rex/issues to report the issue");
                    return;
                }
            }
            
        },
        None=>{
            println!("Please provide a service name. Run `rex help` for the documentation");
            return
        }
    }
}