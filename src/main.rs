use std::env;
use std::process;

use schema_generator::Arguments;

fn main() {
    let args: Vec<String> = env::args().collect();

    let arguments = Arguments::new(&args).unwrap_or_else(| err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = schema_generator::create_config(arguments.clone()) {
        eprintln!("Aplication error: creating config file - {}", e);
        process::exit(1);
    } else {
        println!("Event configuration file successfully created.\n");
    }

    if let Err(e) = schema_generator::create_docs(arguments.clone()) {
        eprintln!("Aplication error: creating doc file - {}", e);
        process::exit(1);
    } else {
        println!("Event documentation file successfully created. Don't forget to add your squad name to the file! (line 16)\n");
    }

    if let Err(e) = schema_generator::create_example(arguments.clone()) {
        eprintln!("Aplication error: creating example file - {}", e);
        process::exit(1);
    } else {
        println!("Example file successfully created.\n")
    }

    if let Err(e) = schema_generator::create_json(arguments.clone()) {
        eprintln!("Aplication error: creating json file - {}", e);
        process::exit(1);
    } else {
        println!("Parameters json file successfully created. Don't forget to add parameters to the generated json!\n")
    }

    if let Err(e) = schema_generator::create_event(arguments.clone()) {
        eprintln!("Aplication error: creating event file - {}", e);
        process::exit(1);
    } else {
        println!("Event json file successfully created.\n")
    }
}
