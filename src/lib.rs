use std::fs::{File, self};
use std::path::Path;
use std::io::Write;
use std::error::Error;

pub fn create_config(arguments: Arguments, base_path: String) -> Result<(), Box<dyn Error>> {
    let base_data = fs::read_to_string("data/config.yaml").expect("Unable to read base config file");
    
    let data = base_data
    .replace("{entity}", &arguments.entity)
    .replace("{action}", &arguments.action);

    let schema_path = format!("{}/config/1.1/mem_community_consume", base_path);
    let final_directory = format!("{}/{}/{}", schema_path, arguments.entity, arguments.action);
    
    let file_name = "event-1.0.config.yaml";
    let final_path = format!("/{}/{}", final_directory, file_name);

    if !Path::new(&final_directory).exists() {
        println!("{} - Directory not found\n. Creating directory\n", final_directory);
        fs::create_dir_all(&final_directory).expect("Unable to create config directory");
    }

    let mut file = File::create(final_path).expect("Unable to create config file");
    file.write_all(&data.as_bytes()).expect("Unable to write config file");

    Ok(())
}

pub fn create_docs(arguments: Arguments, base_path: String) -> Result<(), Box<dyn Error>> {
    let base_data = fs::read_to_string("data/doc.md").expect("Unable to read base doc file");
    let data = base_data.replace("{entity}", &arguments.entity).replace("{action}", &arguments.action);

    let schema_path = format!("{}/doc/1.1/mem_community_consume", base_path);
    let final_directory = format!("{}/{}/{}", schema_path, arguments.entity, arguments.action);
    
    let file_name = "event-1.0.doc.md";
    let final_path = format!("/{}/{}", final_directory, file_name);

    if !Path::new(&final_directory).exists() {
        println!("{} - Directory not found\n. Creating directory\n", final_directory);
        fs::create_dir_all(&final_directory).expect("Unable to create doc directory");
    }

    let mut file = File::create(final_path).expect("Unable to create doc file");
    file.write_all(&data.as_bytes()).expect("Unable to write doc file");

    Ok(())
}

pub fn create_example(arguments: Arguments, base_path: String) -> Result<(), Box<dyn Error>> {
    let base_data = fs::read_to_string("data/example.txt").expect("Unable to read base example file");
    let data = base_data.replace("{entity}", &arguments.entity).replace("{action}", &arguments.action);

    let schema_path = format!("{}/example/1.1/mem_community_consume", base_path);
    let final_directory = format!("{}/{}/{}", schema_path, arguments.entity, arguments.action);
    
    let file_name = "event-1.0.txt";
    let final_path = format!("/{}/{}", final_directory, file_name);

    if !Path::new(&final_directory).exists() {
        println!("{} - Directory not found\n. Creating directory\n", final_directory);
        fs::create_dir_all(&final_directory).expect("Unable to create example directory");
    }

    let mut file = File::create(final_path).expect("Unable to create example file");
    file.write_all(&data.as_bytes()).expect("Unable to write example file");

    Ok(())
}

pub fn create_json(arguments: Arguments, base_path: String) -> Result<(), Box<dyn Error>> {
    let base_data = fs::read_to_string("data/schema.json").expect("Unable to read base schema file");
    let data = base_data.replace("{entity}", &arguments.entity).replace("{action}", &arguments.action);

    let schema_path = format!("{}/json/1.1/mem_community_consume", base_path);
    let final_directory = format!("{}/{}/{}", schema_path, arguments.entity, arguments.action);
    
    let file_name = "event-1.0.schema.json";
    let final_path = format!("/{}/{}", final_directory, file_name);

    if !Path::new(&final_directory).exists() {
        println!("{} - Directory not found\n. Creating directory\n", final_directory);
        fs::create_dir_all(&final_directory).expect("Unable to create schema directory");
    }

    let mut file = File::create(final_path).expect("Unable to create schema file");
    file.write_all(&data.as_bytes()).expect("Unable to write schema file");

    Ok(())
}

pub fn create_event(arguments: Arguments, base_path: String) -> Result<(), Box<dyn Error>> {
    let base_data = fs::read_to_string("data/event.json").expect("Unable to read base event file");
    let data = base_data.replace("{entity}", &arguments.entity).replace("{action}", &arguments.action);

    let schema_path = format!("{}/json/1.1/mem_community_consume", base_path);
    let final_directory = format!("{}/{}", schema_path, arguments.entity);
    
    let file_name = format!("{}_{}.schema.json", arguments.action, arguments.entity);
    let final_path = format!("/{}/{}", final_directory, file_name);

    if !Path::new(&final_directory).exists() {
        println!("{} - Directory not found\n. Creating directory\n", final_directory);
        fs::create_dir_all(&final_directory).expect("Unable to create event directory");
    }

    let mut file = File::create(final_path).expect("Unable to create event file");
    file.write_all(&data.as_bytes()).expect("Unable to write event file");

    Ok(())
}

#[derive(Clone)]
pub struct Arguments {
    pub entity: String,
    pub action: String,
}

impl Arguments {
    pub fn new(args: &[String]) -> Result<Arguments, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let entity = args[1].clone();
        let action = args[2].clone();
    
        Ok(Arguments { entity, action })
    }
}