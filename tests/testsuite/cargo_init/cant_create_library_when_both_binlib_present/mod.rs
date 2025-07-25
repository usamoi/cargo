use crate::prelude::*;
use cargo_test_support::Project;
use cargo_test_support::current_dir;
use cargo_test_support::file;
use cargo_test_support::str;

#[cargo_test]
fn case() {
    let project = Project::from_template(current_dir!().join("in"));
    let project_root = &project.root();

    snapbox::cmd::Command::cargo_ui()
        .arg_line("init --lib")
        .current_dir(project_root)
        .assert()
        .code(101)
        .stdout_eq(str![""])
        .stderr_eq(file!["stderr.term.svg"]);
}
