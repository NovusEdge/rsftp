use crate::ftp_handler::commands;
use ansi_term::Color;
use std::io::{Stdin, StdinLock};

const VALID_COMMANDS: [&str; 26] = [
    "pwd", "lpwd", "cwd", "cd", "cdup", "mkdir", "ls", "dir", "append", "delete", "rm", "rmdir",
    "size", "chmod", "mode", "get", "put", "help", "set", "user", "noop", "exit", "quit",
    "bye", "reset", "verbose",
];

pub fn prompt_user(pwd: &str, host: &str, user: &str) {
    let prompt = format!(
        "[{}]-({}@{})> ",
        Color::White.bold().paint(pwd),
        Color::Cyan.bold().paint(user),
        Color::Blue.bold().paint(host),
    );
    let buffer = String::new();
    let stdin = std::io::stdin();
}

fn input_handler(stdin_handle: StdinLock, command: &str) {}

fn valid_command(cmd: &str) -> bool {
    VALID_COMMANDS
        .to_vec()
        .contains(&cmd.to_lowercase().as_str())
}
