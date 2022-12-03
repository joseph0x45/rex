use std::{self, io::stdin};

fn main() {

    println!("Welcome to Rex CLI 🙂");
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


fn scaffolder( project_name: Option<String>, template: String ){
    //Check if the project name has been provided
    //Ask for it if not
    match project_name {
        Some(project_name)=>{
            let root_folder: String = std::env::current_dir()
                .unwrap()
                .as_os_str()
                .to_str()
                .unwrap()
                .into();
            println!("{}", root_folder);
            let project_folder: String = String::from(std::format!("{}\\{project}", root_folder, project=project_name));
            if std::path::Path::new(&project_folder).is_dir(){
                println!("A folder with the same name already exists");
                return
            }
            

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