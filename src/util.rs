use sloggrs::{debug, info};
use std::{
    fs::{File, create_dir_all, read_dir, remove_dir_all, remove_file},
    io,
    path::{Path, PathBuf},
};

use zip::{CompressionMethod, ZipArchive, ZipWriter, write::SimpleFileOptions};

pub fn extract_zip_file(
    archive_path: &PathBuf,
    output_dir: &PathBuf,
) -> Result<(), std::io::Error> {
    info!("Extracing archive {:?}", archive_path);
    let mut archive = ZipArchive::new(File::open(archive_path)?)?;

    let output_path = Path::new(output_dir);

    if output_dir.exists() {
        if output_dir.is_dir() {
            remove_dir_all(output_path)?;
        } else {
            remove_file(output_path)?;
        }
    }

    create_dir_all(output_dir)?;

    for index in 0..archive.len() {
        let mut file = archive.by_index(index)?;

        let Some(name) = file.enclosed_name() else {
            return Err(std::io::Error::from(io::ErrorKind::InvalidFilename));
        };

        debug!("Extracing {:?} ...", name.as_os_str());

        if file.is_dir() {
            return Err(io::Error::from(io::ErrorKind::IsADirectory));
        }

        let mut out = File::create(output_path.join(&name))?;
        io::copy(&mut file, &mut out)?;
    }

    Ok(())
}

pub fn create_zip_file(input_dir: &Path, archive_path: &Path) -> Result<(), io::Error> {
    info!("Zipping {:?} to {:?}", archive_path, input_dir);

    let mut entries: Vec<PathBuf> = read_dir(input_dir)?
        .map(|entry: Result<std::fs::DirEntry, io::Error>| entry.map(|e| e.path()))
        .collect::<Result<_, _>>()?;

    entries.sort();

    let mut archive = ZipWriter::new(File::create(archive_path)?);
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);

    for path in entries {
        if !path.is_file() {
            return Err(io::Error::from(io::ErrorKind::IsADirectory));
        }

        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            return Err(io::Error::from(io::ErrorKind::InvalidFilename));
        };

        debug!("Adding {:?} ...", name);

        archive.start_file(name, options)?;
        io::copy(&mut File::open(&path)?, &mut archive)?;
    }

    archive.finish()?;

    Ok(())
}
