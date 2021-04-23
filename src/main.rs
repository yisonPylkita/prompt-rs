use chrono::prelude::*;
use std::ffi::OsString;

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

fn main() {
    let mut prompt = String::with_capacity(PROMPT_CAPACITY);
    prompt.push_str(&get_utc_time());
    prompt.push_str(" ");
    prompt.push_str(&with_color(
        get_hostname().to_str().unwrap().to_string(), // TODO: Refactor this
        SHELL_COLOR_GREEN,
    ));
    prompt.push_str(" ");
    prompt.push_str(&get_current_working_directory_path());
    prompt.push_str(" >");

    print!("{}", prompt);
}
