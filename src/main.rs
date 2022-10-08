// indoc!()

// pub mod app;
// pub mod command_line_handling;
pub mod util;
pub mod vcs_commands;

fn main() {
    // let app = app::Application::from([".", "init", "--path", "./test_repos"]);
    // let app = app::Application::default();
    // app.run();
    let curr = std::path::Path::new(".").canonicalize().unwrap();
    println!("{:?}", &curr);
    let binding = curr.join("test_repos").to_owned();
    let root = binding.as_os_str().to_str().unwrap();
    // let vcs_dir = "/home/das/projects/rust/vcs/test_repos/.vcs";
    // std::fs::remove_dir_all(vcs_dir).unwrap_or_else(|_| ());

    // vcs_commands::init::run(root).unwrap();
    util::snapshot::backup(
        &binding,
        &serde_json::from_str("\"c229996a37df312a3ec5a777101ab850813f87da\"").unwrap(),
    )
    .unwrap();
}
