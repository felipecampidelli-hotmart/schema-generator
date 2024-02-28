use std::fs::{File, self};
use std::path::Path;
use std::io::Write;
use std::error::Error;

pub fn create_config(arguments: Arguments, base_path: String) -> Result<(), Box<dyn Error>> {
    let schema_path = format!("{}/config/1.1", base_path);
    let directory = format!("{}/{}/{}/{}", schema_path, arguments.system, arguments.entity, arguments.action);

    let file_data =  FileData { 
        reference_file: "data/config.yaml",
        directory: &directory,
        file_name: "event-1.0.config.yaml"
    };

    file_data.generate_file(arguments)?;

    Ok(())
}

pub fn create_docs(arguments: Arguments, base_path: String) -> Result<(), Box<dyn Error>> {
    let schema_path = format!("{}/doc/1.1", base_path);
    let directory = format!("{}/{}/{}/{}", schema_path, arguments.system, arguments.entity, arguments.action);

    let file_data =  FileData { 
        reference_file: "data/doc.md",
        directory: &directory,
        file_name: "event-1.0.doc.md"
    };

    file_data.generate_file(arguments)?;

    Ok(())
}

pub fn create_example(arguments: Arguments, base_path: String) -> Result<(), Box<dyn Error>> {
    let schema_path = format!("{}/example/1.1", base_path);
    let directory = format!("{}/{}/{}/{}", schema_path, arguments.system, arguments.entity, arguments.action);

    let file_data =  FileData { 
        reference_file: "data/example.txt",
        directory: &directory,
        file_name: "event-1.0.txt"
    };

    file_data.generate_file(arguments)?;

    Ok(())
}

pub fn create_json(arguments: Arguments, base_path: String) -> Result<(), Box<dyn Error>> {
    let schema_path = format!("{}/json/1.1", base_path);
    let directory = format!("{}/{}/{}/{}", schema_path, arguments.system, arguments.entity, arguments.action);

    let file_data =  FileData { 
        reference_file: "data/schema.json",
        directory: &directory,
        file_name: "event-1.0.schema.json"
    };

    file_data.generate_file(arguments)?;

    Ok(())
}

pub fn create_event(arguments: Arguments, base_path: String) -> Result<(), Box<dyn Error>> {
    let schema_path = format!("{}/json/1.1", base_path);
    let directory = format!("{}/{}/{}", schema_path, arguments.system, arguments.entity);
    let file_name = format!("{}_{}.schema.json", arguments.action, arguments.entity);

    let file_data =  FileData { 
        reference_file: "data/event.json",
        directory: &directory,
        file_name: &file_name
    };

    file_data.generate_file(arguments)?;

    Ok(())
}

struct FileData<'a> {
    pub reference_file: &'a str,
    pub directory: &'a str,
    pub file_name: &'a str,
}

impl <'a>FileData<'a> {
    pub fn generate_file(self, arguments: Arguments) -> Result<(), Box<dyn Error>> {
        let base_data = fs::read_to_string(self.reference_file)?;
    
        let data = base_data
            .replace("{system}", &arguments.system)
            .replace("{entity}", &arguments.entity)
            .replace("{action}", &arguments.action);

        let final_path = format!("/{}/{}", self.directory, self.file_name);

        if !Path::new(self.directory).exists() {
            println!("{} - Directory not found. Creating directory.\n", self.directory);
            fs::create_dir_all(self.directory)?;
        }

        let mut file = File::create(final_path)?;
        file.write_all(&data.as_bytes())?;

        Ok(())
    }
}

#[derive(Clone)]
pub struct Arguments {
    pub system: String,
    pub entity: String,
    pub action: String,
}

impl Arguments {
    pub fn new(args: &[String]) -> Result<Arguments, &str> {
        if args.len() < 3 { return Err("Not enough arguments") };

        let system = args[1].clone();
        let entity = args[2].clone();
        let action = args[3].clone();

        Ok(Arguments { system, entity, action })
    }
}