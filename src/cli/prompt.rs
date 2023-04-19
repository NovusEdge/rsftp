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
                commands::help("cd");
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
                commands::help("mkdir");
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
                    Color::Red.paint("[-] Remote/local file not specified properly")
                );
                commands::help("append");
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
                commands::help("delete");
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
                commands::help("rmdir");
                return;
            }
            commands::rmdir(fs, cmd[1]);
        }
        "size" => {
            if cmd.len() < 2 {
                println!("{}", Color::Red.paint("[-] Target file not specified"));
                commands::help("size");
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
        "get" => {
            if cmd.len() < 2 {
                println!("{}", Color::Red.paint("[-] Target file not specified"));
                commands::help("get");
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
                commands::help("put");
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
                commands::help("user");
                return;
            }
            let mut buffer = String::new();
            stdout().flush();
            print!(
                "{}",
                Color::White.bold().paint("Enter passwords for user: ")
            );
            std::io::stdout().flush();
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
            if cmd.len() < 2 {
                commands::print_available_commands();    
            } else {
                commands::help(cmd[1]);
            }
        }
        _ => {
            println!("{}", Color::Red.paint("[-] Invalid Command"));
        }
    };
}
