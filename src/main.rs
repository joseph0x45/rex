mod utils;

use utils::{greet, print_help_message, init};

fn main() {
    match std::env::args().nth(1) {
        Some(arg1)=>{
            match arg1.as_str() {
                "help"=>{
                    print_help_message()
                },
                "init"=>{
                    let template = std::env::args().nth(2);
                    let project_name = std::env::args().nth(3);
                    if let Some(s) = template{
                        init(&s, project_name)
                    }else{
                        println!("You need to specify either you want to use javascript or typescript. Consider running `rex init ts` or `rex init js` \nRead Rex docs here https://github.com/TheWisePigeon/rex#README")
                    }
                },
                "add"=>{
                    match std::env::args().nth(4).unwrap().as_str() {
                        "service"=>{
                            let service_name = std::env::args().nth(5);
                            if let Some(name) = service_name{

                            }else{
                                println!("You did not specify the service name. Read Rex docs here https://github.com/TheWisePigeon/rex#README")
                                
                            }
                        },
                        _=>{
                            println!("The `add` command takes in two parameters: `service` and the service name. Read Rex docs here https://github.com/TheWisePigeon/rex#README")
                        }
                    }
                },
                _=>{
                    println!("Unrecognized command. Read Rex docs here https://github.com/TheWisePigeon/rex#README")
                }
            }
        },
        None=>{
            greet()
        }     
    }
}
