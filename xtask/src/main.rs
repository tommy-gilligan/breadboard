use std::env;
use std::fs;
use std::process::Command;

fn main() {
    let mut args = env::args();
    args.next().unwrap();
    let task_name = args.next().expect("no task name");

    if task_name == "web-serve" {
        // yarn install
        Command::new("yarn")
            .args(["--cwd", "application"])
            .status()
            .expect("failed to execute process");
        // yarn run serve
        Command::new("yarn")
            .args(["--cwd", "application", "run", "serve"])
            .status()
            .expect("failed to execute process")
    } else if task_name == "web-build" {
        let _path = fs::canonicalize("..").unwrap();
        // yarn install
        Command::new("yarn")
            .args(["--cwd", "application"])
            .status()
            .expect("failed to execute process");
        // yarn run build
        Command::new("yarn")
            .args(["--cwd", "application", "run", "build"])
            .status()
            .expect("failed to execute process")
    } else {
        Command::new("cargo")
            .args([
                "run",
                "--package",
                "application",
                "--bin",
                "rp2040",
                "--target",
                "thumbv6m-none-eabi",
            ])
            .status()
            .expect("failed to execute process")
    };
}
