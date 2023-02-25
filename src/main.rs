mod utils;

use std::path::Path;

use utils::{add_service, greet, init, print_help_message};

fn main() {
    match std::env::args().nth(1) {
        Some(arg1) => match arg1.as_str() {
            "help" => print_help_message(),
            "init" => {
                let template = std::env::args().nth(2);
                let project_name = std::env::args().nth(3);
                if let Some(s) = template {
                    init(&s, project_name)
                } else {
                    println!("You need to specify either you want to use javascript or typescript. Consider running `rex init ts` or `rex init js` \nRead Rex docs here https://github.com/TheWisePigeon/rex#README")
                }
            }
            "add" => {
                let current_dir =
                    std::env::current_dir().expect("Error while reading current directory");
                let rex_conf_path = format!(
                    "{}\\rex.conf",
                    current_dir
                        .to_str()
                        .expect("Error while reading current directory")
                );
                if !Path::new(&rex_conf_path).is_file() {
                    println!(
                        "It seems like you are not in a rex generated project, the rex.conf file is missing"
                    );
                    return;
                }
                let rex_conf_content =
                    std::fs::read_to_string(rex_conf_path).expect("Failed to read rex.conf");
                let rex_conf: Vec<&str> = rex_conf_content.split("\n").collect();
                let mut rex_template: &str = "";
                for config in rex_conf {
                    let splitted_line: Vec<&str> = config.split("=").collect();
                    if splitted_line.len() < 2 {
                        println!("Error while reading rex.conf. A key is malformed {config}");
                        return;
                    }
                    if splitted_line[0] == "TEMPLATE" {
                        rex_template = splitted_line[1];
                        break;
                    }
                }
                if rex_template == "" {
                    println!("Failed to read template type from rex.conf");
                    return;
                }
                let add_action = std::env::args().nth(4);
                if let Some(action) = add_action{
                    match action.as_str() {
                        "service" => {
                            let service_name = std::env::args().nth(5);
                            if let Some(name) = service_name {
                            } else {
                                println!("You did not specify the service name. Read Rex docs here https://github.com/TheWisePigeon/rex#README")
                            }
                        }
                        _ => {
                            println!("The `add` command takes in two parameters: `service` and the service name. Read Rex docs here https://github.com/TheWisePigeon/rex#README")
                        }
                    }

                } else{
                    println!("The `add` command takes in two parameters: `service` and the service name. Read Rex docs here https://github.com/TheWisePigeon/rex#README")
                }
            }
            _ => {
                println!("Unrecognized command. Read Rex docs here https://github.com/TheWisePigeon/rex#README")
            }
        },
        None => greet(),
    }
}
