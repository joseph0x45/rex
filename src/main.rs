use std;

fn main() {

    println!("Welcome to Rex CLI 🙂");
    let action: Option<String> = std::env::args().nth(1);
    let template: Option<String> = std::env::args().nth(2);
    match action {
        Some(action)=>{
            match action.as_str() {
                "init"=>{
                    match template {
                        Some(template)=>{
                            match template.as_str() {
                                "js" | "javascript"=>{
                                    println!("Bootstrapping JavaScript template...");
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
            println!("Welcome to the Rex interactive scaffolder");
            return
        }
    }
}
