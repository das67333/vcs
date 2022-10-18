use vcs::app::Application;

pub fn main() {
    // To run with custom arguments:
    // let app = app::Application::from(["aaa", "commit", "-m", "new commit message"]);

    let app = Application::default();
    app.run();
}
