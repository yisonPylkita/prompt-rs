#![feature(pattern)]

use chrono::prelude::*;
use std::{
    ffi::OsString,
    str::{pattern::Pattern, FromStr},
};

const PROMPT_CAPACITY: usize = 256;

type ShellColor = str;
const SHELL_COLOR_RED: &ShellColor = "{red}";
const SHELL_COLOR_GREEN: &ShellColor = "{green}";

enum ShellColorStyle {
    Bg,
    Fg,
}

fn with_color(input: String, color: &ShellColor, style: &ShellColorStyle) -> String {
    let mut result = match style {
        ShellColorStyle::Bg => "%K".to_string(),
        ShellColorStyle::Fg => "%F".to_string(),
    };
    result.push_str(&color.to_string());
    result.push_str(input.as_str());
    match style {
        ShellColorStyle::Bg => result.push_str("%k"),
        ShellColorStyle::Fg => result.push_str("%f"),
    };
    result
}

fn render_last_error_code(error_code: i32) -> String {
    let mut result = String::new();
    result.push('[');
    result.push_str(&error_code.to_string());
    result.push(']');
    if error_code != 0 {
        result = with_color(result, SHELL_COLOR_RED, &ShellColorStyle::Bg);
    }

    result
}

fn get_utc_time() -> String {
    Local::now().format("%H:%M:%S").to_string()
}

fn get_username() -> String {
    whoami::username()
}

fn get_hostname() -> OsString {
    hostname::get().unwrap()
}

fn get_current_working_directory_path() -> String {
    let working_dir = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let home_dir = dirs::home_dir().unwrap().to_str().unwrap().to_string();
    if home_dir.is_prefix_of(working_dir.as_str()) {
        let mut pretty_path = "~/".to_string();
        let sub_path: String = working_dir.chars().skip(home_dir.len() + 1).collect();
        pretty_path.push_str(&sub_path);
        return pretty_path;
    }

    working_dir
}

fn get_git_current_branch_from_libgit2() -> String {
    let repo_handle = git2::Repository::open_bare("./.git");
    if repo_handle.is_err() {
        return "".to_string();
    }
    repo_handle
        .unwrap()
        .head()
        .unwrap()
        .shorthand()
        .unwrap()
        .to_string()
}

fn main() {
    // TODO: Refactor string operations
    // TODO: Refactor code to use anyhow::Result<>
    let start_time = std::time::Instant::now();

    let mut prompt = String::with_capacity(PROMPT_CAPACITY);
    // TODO: write more-proper args parsing. Beware of startup time
    let error_code = std::env::args().nth(2).unwrap().parse::<i32>().unwrap();
    prompt.push_str(&render_last_error_code(error_code));
    prompt.push(' ');
    prompt.push_str(&get_utc_time());
    prompt.push(' ');
    prompt.push_str(&get_username());
    prompt.push('@');
    prompt.push_str(&with_color(
        get_hostname().to_str().unwrap().to_string(),
        SHELL_COLOR_GREEN,
        &ShellColorStyle::Fg,
    ));
    prompt.push(':');
    prompt.push_str(&get_current_working_directory_path());
    prompt.push_str(" | ");
    prompt.push_str(&get_git_current_branch_from_libgit2());
    prompt.push_str(" | > ");
    prompt.push_str(
        &std::time::Instant::now()
            .duration_since(start_time)
            .as_micros()
            .to_string(),
    );
    prompt.push_str("us");

    print!("{}", prompt);
}
