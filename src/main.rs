use std;


fn main() {

    println!("Welcome to Rex CLI 🙂");
    let action = std::env::args().nth(1);
    match action {
        Some(action)=>{
            match action.as_str() {
                "init"=>{
                    println!("init command received");
                    return
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
