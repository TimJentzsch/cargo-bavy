use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};

pub fn create_file_with_content<F, P, C>(folder_name: F, path: P, content: C) -> std::io::Result<()>
where
    F: Into<String>,
    P: Into<String>,
    C: Into<String>,
{
    let full_path = PathBuf::from(format!("{}/{}", folder_name.into(), path.into()));

    // Create the directory if it doesn't exist
    if let Some(dir_path) = full_path.parent() {
        create_dir_all(dir_path).expect("Failed to create parent folder.");
    }

    // Create the file
    let mut file = File::create(full_path)?;
    // Write the content in the file
    file.write_all(content.into().as_bytes())?;

    Ok(())
}
