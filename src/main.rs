use std::ffi::OsString;

use anyhow::Result;

const PROMPT_CAPACITY: usize = 1024;

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

fn get_hostname() -> OsString {
    hostname::get().unwrap()
}

fn main() {
    let mut prompt = String::with_capacity(PROMPT_CAPACITY);
    prompt.push_str("18:55:37 ");
    prompt.push_str(&with_color(
        get_hostname().to_str().unwrap().to_string(), // TODO: Refactor this
        SHELL_COLOR_GREEN,
    ));
    prompt.push_str(" ");

    print!("{}", prompt);
}
