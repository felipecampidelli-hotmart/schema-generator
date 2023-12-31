use std::env;
use std::process;
use std::path::Path;
use dotenv::dotenv;

use schema_generator::Arguments;

fn main() {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();
    
    let base_path = std::env::var("LOCAL_PATH").unwrap();
    if !Path::new(&base_path).exists() {
        let error_message = String::from("Project directory not found!").color_with(91);
        eprintln!("{error_message} Please check if the environment variable LOCAL_PATH is correct.");
        process::exit(1);
    }

    let arguments = Arguments::new(&args).unwrap_or_else(| err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = schema_generator::create_config(arguments.clone(), base_path.clone()) {
        eprintln!("Aplication error: creating config file - {}", e);
        process::exit(1);
    } else {
        println!("Event configuration file successfully created.\n");
    }

    if let Err(e) = schema_generator::create_docs(arguments.clone(), base_path.clone()) {
        eprintln!("Aplication error: creating doc file - {}", e);
        process::exit(1);
    } else {
        println!("Event documentation file successfully created. Don't forget to add your squad name to the file! (line 16)\n");
    }

    if let Err(e) = schema_generator::create_example(arguments.clone(), base_path.clone()) {
        eprintln!("Aplication error: creating example file - {}", e);
        process::exit(1);
    } else {
        println!("Example file successfully created.\n")
    }

    if let Err(e) = schema_generator::create_json(arguments.clone(), base_path.clone()) {
        eprintln!("Aplication error: creating json file - {}", e);
        process::exit(1);
    } else {
        println!("Parameters json file successfully created. Don't forget to add parameters to the generated json!\n")
    }

    if let Err(e) = schema_generator::create_event(arguments.clone(), base_path.clone()) {
        eprintln!("Aplication error: creating event file - {}", e);
        process::exit(1);
    } else {
        println!("Event json file successfully created.\n")
    }
}

trait Color {
    fn color_with(&self, color: i8) -> Self;
}

impl Color for String {
    fn color_with(&self, color: i8) -> Self {
        format!("\x1b[{color}m{self}\x1b[0m")
    }
}