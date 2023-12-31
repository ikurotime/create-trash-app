// src/main.rs

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::error::Error;

fn main() {
    // Fetch the project name from command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: create-trash-app <project-name>");
        std::process::exit(1);
    }

    let project_name = &args[1];

    // Create a new project using the template
    if let Err(err) = create_template_app(project_name) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }

    println!("Project '{}' created successfully!", project_name);
}

// Function to create a new project based on the template
fn create_template_app(project_name: &str) -> Result<(), Box<dyn Error>> {
    let template_path = "template";
    let destination_path = project_name;

    copy_template(template_path, destination_path)?;

    Ok(())
}

// Function to copy the template files to a new project
fn copy_template(template_path: &str, destination_path: &str) -> Result<(), Box<dyn Error>> {
    let template_dir = Path::new(template_path);
    let destination_dir = Path::new(destination_path);

    // Create the new project directory
    fs::create_dir(destination_dir)?;

    // Recursively copy template files to the new project directory
    for entry in fs::read_dir(template_dir)? {
        let entry = entry?;
        let entry_path = entry.path();
        let dest_path = destination_dir.join(entry_path.file_name().unwrap());

        if entry_path.is_dir() {
            fs::create_dir(&dest_path)?;
        } else {
            fs::copy(&entry_path, &dest_path)?;
        }
    }

    Ok(())
}
