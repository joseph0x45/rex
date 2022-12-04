use std::{self, io::stdin};

fn main() {

    println!("
    ██████╗░███████╗██╗░░██╗  ░█████╗░██╗░░░░░██╗
    ██╔══██╗██╔════╝╚██╗██╔╝  ██╔══██╗██║░░░░░██║
    ██████╔╝█████╗░░░╚███╔╝░  ██║░░╚═╝██║░░░░░██║
    ██╔══██╗██╔══╝░░░██╔██╗░  ██║░░██╗██║░░░░░██║
    ██║░░██║███████╗██╔╝╚██╗  ╚█████╔╝███████╗██║
    ╚═╝░░╚═╝╚══════╝╚═╝░░╚═╝  ░╚════╝░╚══════╝╚═╝");
    let action: Option<String> = std::env::args().nth(1);
    let template: Option<String> = std::env::args().nth(2);
    let project_name: Option<String> = std::env::args().nth(3);
    match action {
        Some(action)=>{
            match action.as_str() {
                "init"=>{
                    match template {
                        Some(template)=>{
                            match template.as_str() {
                                "js" | "javascript"=>{
                                    println!("Bootstrapping JavaScript template...");
                                    scaffolder(project_name, template);
                                    return
                                },
                                "ts" | "typescript"=>{
                                    println!("Bootstrapping TypeScript template...");
                                    scaffolder(project_name, template);
                                    return
                                },
                                _=>{
                                    println!("Unrecognized template. Only typescript and javascript templates are supported by Rex for now.");
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
                "help"=>{
                    println!("
                        Welcome to the Rex documentation \n Please refeer to https://github.com/TheWisePigeon/rex/readme.md for the full extended documentation
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


fn scaffolder( project_name: Option<String>, mut template: String ){
    //Templates url match this string
    //https://github.com/TheWisePigeon/rex-<template_name>-template.git https://github.com/TheWisePigeon/rex-js-template.git
    //Check if the project name has been provided
    //Ask for it if not
    match template.as_str() {
        "js" | "javascript" | "JavaScript" =>{
            template = String::from("js");
        },
        "ts" | "typescript" | "TypeScript" =>{
            template = String::from("js")
        },
        _=>{
            return;
        }
    }
    match project_name {
        Some(project_name)=>{
            let root_folder: String = std::env::current_dir()
                .unwrap()
                .as_os_str()
                .to_str()
                .unwrap()
                .into();
            let project_folder: String = String::from(std::format!("{}\\{project}", root_folder, project=project_name));
            if std::path::Path::new(&project_folder).is_dir(){
                println!("A folder with the same name already exists");
                return
            }
            //Clone the content of the corresponding template into the new folder
            std::process::Command::new("git")
                .arg("clone")
                .arg(std::format!("https://github.com/TheWisePigeon/rex-{template}-template.git"))
                .arg(std::format!("{project_name}"))
                .status()
                .expect("Something went wrong");
            //Change current working directory into newly created project folder
            std::env::set_current_dir(&std::path::Path::new(&project_folder)).unwrap();
            //Delete the default remote origin branch
            std::process::Command::new(std::format!("git"))
                .arg("remote")
                .arg("remove")
                .arg("origin")
                .status()
                .expect("Something went wrong while setting up project");

        },
        None=>{
            println!("Enter your project name:");
            let mut project_name: String = String::new() ;
            stdin().read_line(&mut project_name).unwrap();
            println!("{}", project_name);
            return;
        }
    }
}