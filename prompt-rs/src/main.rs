use chrono::prelude::*;
use std::ffi::OsString;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(long)]
    error: i32,
}

const PROMPT_CAPACITY: usize = 256;

type ShellColor = str;
const SHELL_COLOR_RED: &ShellColor = "\\033[0;31m";
const SHELL_COLOR_GREEN: &ShellColor = "\\033[01;32m";
const SHELL_COLOR_RESTORE: &ShellColor = "\\033[0m";

fn with_color(input: String, color: &ShellColor) -> String {
    let mut result = color.to_string();
    result.push_str(input.as_str());
    result.push_str(SHELL_COLOR_RESTORE);
    result
}

fn render_last_error_code(error_code: i32) -> String {
    let mut result = String::new();
    if error_code != 0 {
        result.push_str("%K{red}");
    }
    result.push('[');
    result.push_str(&error_code.to_string());
    result.push(']');
    if error_code != 0 {
        result.push_str("%k");
    }

    result
}

fn get_utc_time() -> String {
    Local::now().format("%H:%M:%S").to_string()
}

fn get_hostname() -> OsString {
    hostname::get().unwrap()
}

fn get_current_working_directory_path() -> String {
    std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

fn get_git_current_branch_from_libgit2() -> String {
    git2::Repository::open(".")
        .unwrap()
        .head()
        .unwrap()
        .shorthand()
        .unwrap()
        .to_string()
}

fn get_git_current_branch_from_git_process() {
    let git_process = std::process::Command::new("/usr/bin/git")
        .arg("branch")
        .arg("--show-current")
        .spawn()
        .unwrap();
    let _ = git_process.wait_with_output().unwrap();
}

fn main() {
    /// libgit2 test
    let start_time = std::time::Instant::now();
    let _ = get_git_current_branch_from_libgit2();
    println!(
        "git branch --show-current equivalent in libgit2 took {}us",
        std::time::Instant::now()
            .duration_since(start_time)
            .as_micros()
    );

    /// git process test
    let start_time = std::time::Instant::now();
    let _ = get_git_current_branch_from_git_process();
    println!(
        "git branch --show-current as new process took {}us",
        std::time::Instant::now()
            .duration_since(start_time)
            .as_micros()
    );

    let opt = Opt::from_args();

    let mut prompt = String::with_capacity(PROMPT_CAPACITY);
    prompt.push_str(&render_last_error_code(opt.error));
    prompt.push(' ');
    prompt.push_str(&get_utc_time());
    prompt.push_str(" ");
    prompt.push_str(&with_color(
        get_hostname().to_str().unwrap().to_string(), // TODO: Refactor this
        SHELL_COLOR_GREEN,
    ));
    prompt.push_str(" ");
    prompt.push_str(&get_current_working_directory_path());
    prompt.push_str(" | ");
    prompt.push_str(&get_git_current_branch_from_libgit2());
    prompt.push_str(" | > ");

    print!("{}", prompt);
}
