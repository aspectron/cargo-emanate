#![allow(dead_code)]

use crate::prelude::*;
use console::style;
use flate2::read::GzDecoder;
use serde::Serialize;
use std::fs::File;
use std::io::Read;
use std::io::{Seek, Write};
use std::path::Path;
use std::path::PathBuf;
use tar::Archive as TarArchive;
use walkdir::{DirEntry, WalkDir};
use zip::result::ZipError;
use zip::write::FileOptions;

pub async fn extract(
    file: &async_std::path::PathBuf,
    dir: &async_std::path::PathBuf,
) -> Result<()> {
    let file_str = file.clone().into_os_string().into_string()?;
    // println!("extracting file: {} to {}", file_str, dir_str);
    if file_str.ends_with(".tar.gz") || file_str.ends_with(".tgz") {
        extract_tar_gz(&file.into(), &dir.into())?;
    } else if file_str.ends_with(".zip") {
        extract_zip(&file.into(), &dir.into()).await?;
    } else {
        return Err(format!("extract(): unsupported file type: {file_str}").into());
    }

    Ok(())
}

fn extract_tar_gz(file: &PathBuf, dir: &PathBuf) -> Result<()> {
    // https://rust-lang-nursery.github.io/rust-cookbook/compression/tar.html
    let tar_gz = std::fs::File::open(file)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = TarArchive::new(tar);
    archive.unpack(dir)?;
    Ok(())
}

async fn extract_zip(file: &PathBuf, dir: &PathBuf) -> Result<()> {
    let file_reader = std::fs::File::open(file).unwrap();
    let mut archive = zip::ZipArchive::new(file_reader).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => std::path::Path::new(dir).join(path), //.to_owned(),
            None => continue,
        };

        // {
        //     let comment = file.comment();
        //     if !comment.is_empty() {
        //         println!("File {} comment: {}", i, comment);
        //     }
        // }

        if (*file.name()).ends_with('/') {
            // println!("File {} extracted to \"{}\"", i, outpath.display());
            std::fs::create_dir_all(&outpath).unwrap();
        } else {
            // println!(
            //     "File {} extracted to \"{}\" ({} bytes)",
            //     i,
            //     outpath.display(),
            //     file.size()
            // );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = std::fs::File::create(&outpath).unwrap();
            std::io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    Ok(())
}

fn zip_folder<T>(
    nb_files: usize,
    filename: &str,
    _path: &Path,
    it: &mut dyn Iterator<Item = DirEntry>,
    // prefix: &str,
    prefix: &Path,
    writer: T,
    method: zip::CompressionMethod,
) -> Result<()>
where
    T: Write + Seek,
{
    let mut count: usize = 0;
    let mut bytes: usize = 0;
    let filename = style(filename).cyan();

    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(prefix).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            // println!("adding file {:?} as {:?} ...", path, name);
            #[allow(deprecated)]
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            bytes += buffer.len();
            buffer.clear();
            // zip.fl
        } else if !name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            // println!("adding dir {:?} as {:?} ...", path, name);
            #[allow(deprecated)]
            zip.add_directory_from_path(name, options)?;
        }

        count += 1;
        let pos = count as f64 / nb_files as f64 * 100.0;
        let percent = style(format!("{pos:1.2}%")).cyan();
        let size = style(format!("{:1.2} Mb", bytes as f64 / 1024.0 / 1024.0)).cyan();
        let files = style(format!("{count}/{nb_files} files")).cyan();
        log_state!(
            "Compressing",
            "... {filename}: {percent} - {files} - {size} "
        );
    }

    log_state_clear();
    zip.finish()?;

    Ok(())
}

pub fn compress_folder(
    src_dir: &std::path::Path,
    dst_file: &std::path::Path,
    archive: Archive,
) -> Result<()> {
    if !Path::new(src_dir).is_dir() {
        return Err(ZipError::FileNotFound.into());
    }

    let algorithm = archive.algorithm.unwrap_or_default();
    let subfolder = archive.subfolder.unwrap_or(true);

    log_info!("Archive", "compressing ({})", algorithm.to_string());
    let method: zip::CompressionMethod = algorithm.into();

    let path = Path::new(dst_file);
    let file = File::create(path).unwrap();

    let walkdir = WalkDir::new(src_dir);
    let it = walkdir.into_iter();
    let mut nb_files = 0;
    for _ in it {
        nb_files += 1;
    }

    let walkdir = WalkDir::new(src_dir);
    let it = walkdir.into_iter();

    let prefix = if subfolder {
        src_dir.parent().unwrap()
    } else {
        src_dir
    };

    zip_folder(
        nb_files,
        dst_file.file_name().unwrap().to_str().unwrap(),
        path,
        &mut it.filter_map(|e| e.ok()),
        prefix,
        file,
        method,
    )?;

    Ok(())
}

// -----------------------------------------------------------------------------

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum Algorithm {
    Store,
    BZip2,
    #[default]
    Deflate,
    ZStd,
}

impl From<Algorithm> for zip::CompressionMethod {
    fn from(algorithm: Algorithm) -> zip::CompressionMethod {
        match algorithm {
            Algorithm::Store => zip::CompressionMethod::Stored,
            Algorithm::BZip2 => zip::CompressionMethod::Bzip2,
            Algorithm::Deflate => zip::CompressionMethod::Deflated,
            Algorithm::ZStd => zip::CompressionMethod::Zstd,
        }
    }
}

impl ToString for Algorithm {
    fn to_string(&self) -> String {
        match self {
            Algorithm::Store => "STORE",
            Algorithm::BZip2 => "BZIP2",
            Algorithm::Deflate => "DEFLATE",
            Algorithm::ZStd => "ZSTD",
        }
        .into()
    }
}

/// Zip Archive compression modes.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
// #[serde(deny_unknown_fields)]
pub struct Archive {
    pub algorithm: Option<Algorithm>,
    pub subfolder: Option<bool>,
}
