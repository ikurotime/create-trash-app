use std::fs;
use std::path::PathBuf;

fn copy_template(template_path: &str, destination_path: &str) -> Result<(), std::io::Error> {
    let template_dir = PathBuf::from(template_path);
    let destination_dir = PathBuf::from(destination_path);

    fs::create_dir_all(&destination_dir)?;

    for entry in fs::read_dir(&template_dir)? {
        let entry = entry?;
        let entry_path = entry.path();
        let destination_entry_path = destination_dir.join(entry_path.file_name().unwrap());

        if entry_path.is_dir() {
            fs::create_dir_all(&destination_entry_path)?;
        } else {
            fs::copy(&entry_path, &destination_entry_path)?;
        }
    }

    Ok(())
}
pub fn create_template_app(project_name: &str) -> Result<(), std::io::Error> {
    let template_path = "template";
    let destination_path = project_name;

    copy_template(template_path, destination_path)
}
// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_create_template_app() {
        let project_name = "test_project";
        let result = create_template_app(project_name);
        assert!(result.is_ok());

        // Clean up: remove the test project directory
        let _ = fs::remove_dir_all(project_name);
    }
}