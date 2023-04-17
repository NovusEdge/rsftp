use crate::ftp_handler::commands;
use ansi_term::Color;
use std::io::{Stdin, StdinLock};

const VALID_COMMANDS: [&str; 24] = [
    "pwd", "lpwd", "cwd", "cd", "cdup", "mkdir", "ls", "dir", "append", "delete", "rm", "rmdir",
    "size", "chmod", "mode", "get", "put", "help", "user", "noop", "exit", "quit", "bye",
    "verbose",
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

fn valid_command(cmd: &str) -> bool {
    VALID_COMMANDS
        .to_vec()
        .contains(&cmd.to_lowercase().as_str())
}

fn input_handler(stdin_handle: StdinLock, command: &str) {

}


