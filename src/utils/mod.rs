use std::io;

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

pub fn verify_template(){

}

pub fn init( argument: &str, project_name: Option<String> ){
    if argument!="js" && argument!="ts" {
        println!("Invalid argument \"{argument}\". Only `ts` and `js` are valid arguments for the init command");
        return;
    }
    if let Some(project_name) = project_name{
        println!("{project_name}")
    }else{
        let mut project_name: String = String::new();
        println!("What is your project name? Leave blank to use current directory(Works for empty directories only)");
        io::stdin().read_line(&mut project_name).expect("Failed to read project name");
        
    }
}