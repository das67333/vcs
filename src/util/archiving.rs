use std::fs::{create_dir_all, File};
use std::io::{copy, prelude::Read, Write};
use std::path::Path;
use zip::result::{ZipError, ZipResult};
use zip::write::FileOptions;

const PRESERVE_PERMISSIONS: bool = false;

/// Compress directory with ".vcs" directory excluded
pub fn zip(src_dir: &Path, dst_archive: &Path) -> ZipResult<()> {
    if !src_dir.is_dir() {
        return Err(ZipError::FileNotFound);
    }

    let mut zip_writer = zip::ZipWriter::new(File::create(dst_archive)?);
    let mut buffer = Vec::new();
    for entry in walkdir::WalkDir::new(src_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        // unwrap: "src_dir" is always a prefix of "path"
        let name_as_path = path.strip_prefix(src_dir).unwrap();
        // unwrap: "path" must be at least Unicode
        if name_as_path
            .components()
            .any(|x| x.as_os_str().to_str().unwrap() == ".vcs")
        {
            continue;
        }
        let name_as_str = name_as_path.as_os_str().to_str().unwrap();

        // options are changed in unix and PRESERVE_PERMISSIONS case
        #[allow(unused_mut)]
        let mut options = FileOptions::default();

        if PRESERVE_PERMISSIONS {
            #[cfg(unix)]
            if let Ok(metadata) = path.metadata() {
                use std::os::unix::fs::PermissionsExt;
                options = options.unix_permissions(metadata.permissions().mode());
            }
        }

        if path.is_file() {
            zip_writer.start_file(name_as_str, options)?;
            let mut file = File::open(path)?;
            file.read_to_end(&mut buffer)?;
            zip_writer.write_all(&*buffer)?;
            buffer.clear();
        } else if !name_as_path.as_os_str().is_empty() {
            zip_writer.add_directory(name_as_str, FileOptions::default())?;
        }
    }
    zip_writer.finish()?;
    Ok(())
}

/// Extract zip file
pub fn unzip(src_archive: &Path, dst_dir: &Path) -> ZipResult<()> {
    let file = File::open(src_archive)?;

    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        // unwrap: "file" is always inside "archive"
        let path = dst_dir.join(file.enclosed_name().unwrap());
        // '/' is a dirty hack in zip library and should not be replaced with std::path::MAIN_SEPARATOR
        if file.name().ends_with('/') {
            create_dir_all(&path)?;
        } else {
            if let Some(par) = path.parent() {
                if !par.exists() {
                    create_dir_all(&par)?;
                }
            }
            let mut outfile = File::create(&path)?;
            copy(&mut file, &mut outfile)?;
        }

        if PRESERVE_PERMISSIONS {
            #[cfg(unix)]
            {
                use std::fs::{set_permissions, Permissions};
                use std::os::unix::fs::PermissionsExt;
                if let Some(mode) = file.unix_mode() {
                    set_permissions(&path, Permissions::from_mode(mode))?;
                }
            }
        }
    }
    Ok(())
}
