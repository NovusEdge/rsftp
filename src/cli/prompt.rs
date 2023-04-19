use crate::ftp_handler::{client::IsFtpStream, commands};
use ansi_term::Color;
use std::fs::File;
use std::io::{stdout, Write};

pub fn prompt_user<T: IsFtpStream>(pwd: &str, host: &str, user: &str, fs: &mut T) -> String {
    let prompt = format!(
        "[{}]-({}@{})> ",
        Color::White.bold().paint(pwd),
        Color::Cyan.bold().paint(user),
        Color::Blue.bold().paint(host),
    );

    print!("{}", prompt);
    stdout().flush().unwrap();

    let mut buffer = String::new();
    let mut cache = String::from(user);
    let stdin = std::io::stdin();
    stdin.read_line(&mut buffer).unwrap();
    input_handler(buffer.as_str(), fs, &mut cache);
    cache
}

fn input_handler<T: IsFtpStream>(command: &str, fs: &mut T, cache: &mut String) {
    if command.len() == 0 || command == "" || command == "\n" {
        commands::noop(fs);
        return;
    }

    let cmd = command
        .split_whitespace()
        .into_iter()
        .collect::<Vec<&str>>();

    // check the command first!
    match cmd[0] {
        "pwd" => {
            commands::pwd(fs);
        }
        "lpwd" => {
            commands::lpwd();
        }
        "cd" | "cwd" => {
            if cmd.len() < 2 {
                println!("{}", Color::Red.paint("[-] Remote directory not specified"));
                println!(
                    "{}: {}",
                    Color::Yellow.dimmed().bold().paint("USAGE for cd|cwd"),
                    Color::Yellow.dimmed().paint("cd|cwd REMOTE_DIR")
                );
                return;
            }
            let remote_dir = cmd[1];
            commands::cwd(fs, remote_dir);
        }
        "cdup" => {
            commands::cdup(fs);
        }
        "mkdir" => {
            if cmd.len() < 2 {
                println!("{}", Color::Red.paint("[-] Target directory not specified"));
                println!(
                    "{}: {}",
                    Color::Yellow.dimmed().bold().paint("USAGE for mkdir"),
                    Color::Yellow.dimmed().paint("mkdir REMOTE_DIR")
                );
                return;
            }

            let pathname = cmd[1];
            commands::mkdir(fs, pathname);
        }
        "ls" | "dir" => {
            if cmd.len() < 2 {
                commands::ls(fs, ".");
                return;
            }
            let target = cmd[1];
            commands::ls(fs, target);
        }
        "append" => {
            if cmd.len() < 3 {
                println!(
                    "{}",
                    Color::Red.paint("[-] Remote/LOCAL_FILE not specified properly")
                );
                println!(
                    "{}: {}",
                    Color::Yellow.dimmed().bold().paint("USAGE for append"),
                    Color::Yellow
                        .dimmed()
                        .paint("append LOCAL_FILE REMOTE_FILE")
                );
                return;
            }

            match File::open(cmd[1]) {
                Ok(mut file) => {
                    let remote_file = cmd[2];
                    commands::append(fs, &mut file, remote_file);
                }
                Err(e) => {
                    println!("{}", Color::Red.paint(format!("[-] {}", e)));
                    return;
                }
            }
        }
        "delete" | "rm" => {
            if cmd.len() < 2 {
                println!("{}", Color::Red.paint("[-] Target file(s) not specified"));
                println!(
                    "{}: {}",
                    Color::Yellow.dimmed().bold().paint("USAGE for delete|rm"),
                    Color::Yellow.dimmed().paint("delete|rm REMOTE_FILES...")
                );
                return;
            }
            if cmd.len() == 2 {
                commands::rm(fs, cmd[1]);
                return;
            }

            for i in cmd.iter().skip(1) {
                commands::rm(fs, i);
            }
        }
        "rmdir" => {
            if cmd.len() < 2 {
                println!("{}", Color::Red.paint("[-] Target directory not specified"));
                println!(
                    "{}: {}",
                    Color::Yellow.dimmed().bold().paint("USAGE for rmdir"),
                    Color::Yellow.dimmed().paint("rmdir REMOTE_DIR")
                );
                return;
            }
            commands::rmdir(fs, cmd[1]);
        }
        "size" => {
            if cmd.len() < 2 {
                println!("{}", Color::Red.paint("[-] Target file not specified"));
                println!(
                    "{}: {}",
                    Color::Yellow.dimmed().bold().paint("USAGE for size"),
                    Color::Yellow.dimmed().paint("size REMOTE_FILES...")
                );
                return;
            }
            if cmd.len() == 2 {
                commands::size(fs, cmd[1]);
                return;
            }

            for i in cmd.iter().skip(1) {
                commands::size(fs, i);
            }
        }
        "chmod" => {
            todo!()
            // commands::chmod(fs, ...);
        }
        "get" => {
            if cmd.len() < 2 {
                println!("{}", Color::Red.paint("[-] Target file not specified"));
                println!(
                    "{}: {}",
                    Color::Yellow.dimmed().bold().paint("USAGE for get"),
                    Color::Yellow.dimmed().paint("get REMOTE_FILE [LOCAL_FILE]")
                );
                return;
            }
            if cmd.len() == 2 {
                match File::create(cmd[1]) {
                    Ok(mut file) => {
                        commands::get(fs, cmd[1], &mut file);
                    }
                    Err(e) => {
                        println!("{}", Color::Red.paint(format!("[-] {}", e)));
                        return;
                    }
                }
                return;
            }

            match File::open(cmd[2]) {
                Ok(mut file) => {
                    commands::get(fs, cmd[1], &mut file);
                }
                Err(e) => {
                    println!("{}", Color::Red.paint(format!("[-] {}", e)));
                    return;
                }
            }
        }
        "put" => {
            if cmd.len() < 2 {
                println!("{}", Color::Red.paint("[-] Target file not specified"));
                println!(
                    "{}: {}",
                    Color::Yellow.dimmed().bold().paint("USAGE for put"),
                    Color::Yellow.dimmed().paint("put LOCAL_FILE [REMOTE_FILE]")
                );
                return;
            }
            if cmd.len() == 2 {
                match File::open(cmd[1]) {
                    Ok(mut file) => {
                        commands::put(fs, &mut file, cmd[1]);
                    }
                    Err(e) => {
                        println!("{}", Color::Red.paint(format!("[-] {}", e)));
                        return;
                    }
                }
                return;
            }

            match File::open(cmd[1]) {
                Ok(mut file) => {
                    commands::put(fs, &mut file, cmd[2]);
                }
                Err(e) => {
                    println!("{}", Color::Red.paint(format!("[-] {}", e)));
                    return;
                }
            }
            return;
        }
        "user" => {
            if cmd.len() < 2 {
                println!("{}", Color::Red.paint("[-] User not specified"));
                println!(
                    "{}: {}",
                    Color::Yellow.dimmed().bold().paint("USAGE for user"),
                    Color::Yellow.dimmed().paint("user USER")
                );
                return;
            }
            let mut buffer = String::new();
            print!(
                "{}",
                Color::White.bold().paint("Enter passwords for user: ")
            );
            std::io::stdin().read_line(&mut buffer).unwrap();
            commands::user(fs, cmd[1], buffer.as_str(), cache);
        }
        "noop" => {
            commands::noop(fs);
        }
        "bye" | "quit" | "exit " => {
            commands::quit(fs);
            std::process::exit(0);
        }
        "help" => {
            commands::help();
        }
        _ => {
            println!("{}", Color::Red.paint("[-] Invalid Command"));
        }
    };
}
