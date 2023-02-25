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
                        println!("You need to specify either you want to use javascript or typescript. Consider running `rex init ts` or `rex init js`")
                    }

                },
                _=>{

                }
            }
        },
        None=>{
            greet()
        }     
    }
}
