use std::env;
use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::{Write};
use chrono::Local;
use serde_json;
use serde::{Serialize, Deserialize};

// Todo object
#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    text: String,
    created: u64,
}

impl Todo {
    pub fn new(text: &str) -> Todo {
        let date: String = String::from(Local::now().format("%Y%m%d%H%M%S").to_string());
        let created: u64 = date.parse().unwrap();

        Todo {
            text: text.to_string(),
            created: created,
        }
    }
}

fn add(text: &str) -> std::io::Result<()> {
    // output json file path
    let file_path = "rustodo.json";
    let path = Path::new(file_path);

    // initialize the json file if not that is existing
    if !path.exists() {
        let mut f = File::create(file_path)?;
        let input = String::from("[]");
        f.write_all(input.as_bytes());
    }
 
    // deserialized (json to string)
    let input = fs::read_to_string(file_path).expect("JSON Read Failed!");
    let mut deserialized: Vec<Todo> = serde_json::from_str(&input).unwrap();
  
    // add new text
    let todo = Todo::new(text);
    deserialized.push(todo);

    // serialized (string to json)
    let serialized = serde_json::to_string(&deserialized).unwrap();
    
    // Write the result to the json file
    let mut f = File::create(file_path)?;
    let input = String::from(serialized);
    f.write_all(input.as_bytes())?;
    
    Ok(())
}

fn del(text: &str) -> std::io::Result<()> {
    // output json file path
    let file_path = "rustodo.json";
    let path = Path::new(file_path);
    
    // Check the file existing
    if !path.exists() {
        eprintln!("file does not exists!");
    } 
    
    // deserialized (json to string)
    let input = fs::read_to_string(file_path).expect("JSON Read Failed!");
    let mut deserialized: Vec<Todo> = serde_json::from_str(&input).unwrap();
  
    // delete the specified element
    let index: usize = text.parse().unwrap();
    deserialized.remove(index);

    // serialized (string to json)
    let serialized = serde_json::to_string(&deserialized).unwrap();
    
    // Write to json file
    let mut f = File::create(file_path)?;
    let input = String::from(serialized);
    f.write_all(input.as_bytes())?;

    Ok(())
}

fn clear() -> std::io::Result<()> {
    // output json file path
    let file_path = "rustodo.json";

    // delete everything
    let mut f = File::create(file_path)?;
    let input = String::from("[]");
    f.write_all(input.as_bytes());
 
    Ok(())
}

fn print() -> std::io::Result<()> {
    // output json file path
    let file_path = "rustodo.json";
    let path = Path::new(file_path);

    // initialize the json file if not that is existing
    if !path.exists() {
        eprintln!("file does not exists!");
    }
 
    // deserialized (json to string)
    let input = fs::read_to_string(file_path).expect("JSON Read Failed!");
    let mut deserialized: Vec<Todo> = serde_json::from_str(&input).unwrap();
  
    // serialized (string to json)
    let serialized = serde_json::to_string(&deserialized).unwrap();

    println!("{}", serialized);

    Ok(())
}

fn main() {
    // command line arguments
    let args: Vec<String> = env::args().collect();
    let arg1: &str = &args[1];
    
    if arg1 == "add" {
        let arg2: &str = &args[2];
        add(arg2);
    } else if arg1 == "del" {
        let arg2: &str = &args[2];
        del(arg2);
    } else if arg1 == "clear" {
        clear();
    } else if arg1 == "print" {
        print();
    } else {
        println!("invalid arguments!");
    }
}






