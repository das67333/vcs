use std::fs::{create_dir_all, File};
use std::io::{copy, prelude::Read, Write};
use std::path::Path;
use walkdir::WalkDir;
use zip::result::ZipError;
use zip::write::FileOptions;
use zip::ZipWriter;

const PRESERVE_PERMISSIONS: bool = false;

/// Compresses the given directory with ".vcs" subdirectory excluded
pub fn zip(src_dir: &Path, dst_archive: &Path) -> Result<(), ZipError> {
    if !src_dir.is_dir() {
        panic!("zip can be applyed to directories only");
    }

    let mut zip_writer = ZipWriter::new(File::create(dst_archive)?);
    let mut buffer = Vec::new();
    // ignore all inaccessible subdirectories
    for entry in WalkDir::new(src_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        // unwrap: "src_dir" is always a prefix of "path"
        let name_as_path = path.strip_prefix(src_dir).unwrap();
        if name_as_path
            .components()
            .any(|x| x.as_os_str().to_str() == Some(".vcs"))
        {
            continue;
        }
        // unwrap: "name_as_path" must be at least Unicode
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

/// Extracts the given zip archive
pub fn unzip(src_archive: &Path, dst_dir: &Path) -> Result<(), ZipError> {
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
                if !par.try_exists()? {
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_archiving() -> Result<(), std::io::Error> {
        let path_dir = std::path::Path::new("temp_dir");
        let path_file = path_dir.join("temp_file.txt");
        let path_archive = std::path::Path::new("temp_archive");
        let contents = "abbbcc";

        std::fs::create_dir(&path_dir)?;
        std::fs::write(&path_file, contents)?;
        super::zip(&path_dir, &path_archive)?;
        std::fs::remove_dir_all(&path_dir)?;
        super::unzip(&path_archive, &path_dir)?;
        std::fs::remove_file(&path_archive)?;
        assert_eq!(std::fs::read_to_string(&path_file)?, contents);
        std::fs::remove_dir_all(&path_dir)?;
        Ok(())
    }
}
