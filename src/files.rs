use std::{
    fs::{create_dir_all, remove_dir_all, File},
    io::Write,
    path::PathBuf,
};

use anyhow::Result;
use fs_extra::dir::{self, CopyOptions};

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

/// Copy `source_folder` into `target_folder` and replace the previous folder.
///
/// Note that the whole `source_folder` will be copied, not only the contents
pub fn replace_folder<S, T>(source_folder: S, target_folder: T) -> Result<()>
where
    S: Into<String>,
    T: Into<String>,
{
    let source_path = PathBuf::from(source_folder.into());
    let target_dir_path = PathBuf::from(target_folder.into());
    let target_path = target_dir_path.join(source_path.file_name().unwrap());

    if target_path.exists() {
        remove_dir_all(&target_path)?;
    }

    if source_path.exists() {
        dir::copy(source_path, target_dir_path, &CopyOptions::new())?;
    }

    Ok(())
}
