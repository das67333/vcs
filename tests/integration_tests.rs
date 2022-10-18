use std::env::current_dir;
use std::fs::{create_dir, remove_dir_all, write};
use std::io::Error;
use std::path::Path;
use vcs::util::vcs_state::VcsState;
use vcs::vcs_commands::*;

fn fill_directory(path: &Path) -> Result<(), Error> {
    write(path.join("1.txt"), "1")?;
    let inner = path.join("inner");
    create_dir(&inner)?;
    write(inner.join("2.txt"), "2")?;
    Ok(())
}

fn change_contents(path: &Path) -> Result<(), Error> {
    write(path.join("1.txt"), "3")?;
    Ok(())
}

fn restore_contents(path: &Path) -> Result<(), Error> {
    write(path.join("1.txt"), "1")?;
    Ok(())
}

fn add_contents(path: &Path) -> Result<(), Error> {
    write(path.join("3.txt"), "3")?;
    Ok(())
}

#[test]
fn test_init1() -> Result<(), Error> {
    let repos_str = "test_init1";
    let repos_path = current_dir()?.join(&repos_str);
    assert!(!repos_path.try_exists()?, "{:?}", repos_path);

    create_dir(&repos_path)?;
    assert!(init::run(&repos_path).is_ok());
    VcsState::assert_validity(&repos_path);

    remove_dir_all(&repos_path)?;
    Ok(())
}

#[test]
fn test_init2() -> Result<(), Error> {
    let repos_str = "test_init2";
    let repos_path = current_dir()?.join(&repos_str);
    assert!(!repos_path.try_exists()?, "{:?}", repos_path);

    create_dir(&repos_path)?;
    assert!(fill_directory(&repos_path).is_ok());
    assert!(init::run(&repos_path).is_ok());
    VcsState::assert_validity(&repos_path);

    remove_dir_all(&repos_path)?;
    Ok(())
}

#[test]
fn test_status() -> Result<(), Error> {
    let repos_str = "test_status";
    let repos_path = current_dir()?.join(&repos_str);
    assert!(!repos_path.try_exists()?, "{:?}", repos_path);

    assert!(init::run(&repos_path).is_ok());
    VcsState::assert_validity(&repos_path);

    const NO_CHANGES: &str = "No changes to be committed";
    assert_eq!(status::run(&repos_path).ok().unwrap(), NO_CHANGES);
    VcsState::assert_validity(&repos_path);
    assert!(fill_directory(&repos_path).is_ok());
    assert_ne!(status::run(&repos_path).ok().unwrap(), NO_CHANGES);
    VcsState::assert_validity(&repos_path);

    remove_dir_all(&repos_path)?;
    Ok(())
}

#[test]
fn test_complex() -> Result<(), Error> {
    let repos_str = "test_complex1";
    let repos_path = current_dir()?.join(&repos_str);
    assert!(!repos_path.try_exists()?, "{:?}", repos_path);

    assert!(init::run(&repos_path).is_ok());
    VcsState::assert_validity(&repos_path);
    assert!(commit::run(&repos_path, "hi").is_err());
    VcsState::assert_validity(&repos_path);
    assert!(fill_directory(&repos_path).is_ok());

    let init_hash = new_branch::run(&repos_path, "dev").ok().unwrap();
    let init_hash = &init_hash[(init_hash.len() - 6)..];

    VcsState::assert_validity(&repos_path);
    assert!(new_branch::run(&repos_path, "dev").is_err());
    VcsState::assert_validity(&repos_path);

    assert!(jump::run(&repos_path, &Some("master".to_owned()), &None).is_err());

    assert!(commit::run(&repos_path, "hi").is_ok());
    VcsState::assert_validity(&repos_path);

    assert!(jump::run(&repos_path, &None, &None).is_err());
    VcsState::assert_validity(&repos_path);
    assert!(jump::run(
        &repos_path,
        &Some("master".to_owned()),
        &Some(init_hash.to_owned())
    )
    .is_err());
    assert!(jump::run(&repos_path, &Some("master".to_owned()), &None).is_ok());
    VcsState::assert_validity(&repos_path);
    assert!(jump::run(&repos_path, &Some("dev".to_owned()), &None).is_ok());
    VcsState::assert_validity(&repos_path);
    assert!(jump::run(&repos_path, &None, &Some(init_hash.to_owned())).is_ok());
    VcsState::assert_validity(&repos_path);

    assert!(change_contents(&repos_path).is_ok());
    assert!(merge::run(&repos_path, "dev").is_err());
    VcsState::assert_validity(&repos_path);
    assert!(restore_contents(&repos_path).is_ok());
    assert!(add_contents(&repos_path).is_ok());
    assert!(merge::run(&repos_path, "dev").is_err());
    VcsState::assert_validity(&repos_path);
    assert!(commit::run(&repos_path, "hi2").is_ok());
    VcsState::assert_validity(&repos_path);
    assert!(merge::run(&repos_path, "dev").is_ok());
    VcsState::assert_validity(&repos_path);

    remove_dir_all(&repos_path)?;
    Ok(())
}
