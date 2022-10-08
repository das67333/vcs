// indoc!()

// pub mod app;
// pub mod command_line_handling;
pub mod util;
pub mod vcs_commands;

fn main() {
    // let app = app::Application::from([".", "init", "--path", "./test_repos"]);
    // let app = app::Application::default();
    // app.run();
    
    let root = "/home/das/projects/rust/vcs/test_repos";
    let vcs_dir = "/home/das/projects/rust/vcs/test_repos/.vcs";
    std::fs::remove_dir_all(vcs_dir).unwrap_or_else(|_| ());
    
    vcs_commands::init::run(root).unwrap();

}
