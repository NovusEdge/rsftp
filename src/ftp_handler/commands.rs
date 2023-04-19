use crate::ftp_handler::client::IsFtpStream;
use ansi_term::Color;
use std::fs::File;
use std::io::prelude::*;
use suppaftp::FtpError;

pub fn pwd<T: IsFtpStream>(fs: &mut T) {
    match fs.pwd() {
        Ok(s) => {
            println!(
                "{}: {}",
                Color::White.dimmed().paint("Remote directory"),
                Color::Green.bold().dimmed().paint(s)
            );
        }
        Err(e) => {
            print_error(e);
        }
    };
}

pub fn lpwd() {
    match std::env::current_dir() {
        Ok(s) => {
            println!(
                "Local Directory: {}",
                Color::Green.dimmed().paint(s.display().to_string())
            );
        }
        Err(e) => {
            println!("{}", Color::Red.paint(format!("[-]: {}", e)));
        }
    };
}

pub fn cwd<T: IsFtpStream>(fs: &mut T, remote_dir: &str) {
    match fs.cwd(remote_dir) {
        Ok(()) => {
            println!(
                "{}: {}",
                Color::White.dimmed().paint("Changed to"),
                Color::Green.bold().dimmed().paint(remote_dir)
            );
        }
        Err(e) => {
            print_error(e);
        }
    };
}

pub fn cdup<T: IsFtpStream>(fs: &mut T) {
    match fs.cdup() {
        Ok(()) => {}
        Err(e) => {
            print_error(e);
        }
    };
}

pub fn ls<T: IsFtpStream>(fs: &mut T, remote_dir: &str) {
    match fs.list(Some(remote_dir)) {
        Ok(s) => {
            println!(
                "Listing directory: {}",
                Color::Blue.bold().paint(remote_dir)
            );
            for i in s {
                println!("{}", i);
            }
        }
        Err(e) => {
            print_error(e);
        }
    };
}

pub fn mkdir<T: IsFtpStream>(fs: &mut T, new_dir: &str) {
    match fs.mkdir(new_dir) {
        Ok(()) => {
            println!("{}", Color::Green.paint("[+]: Success!"));
        }
        Err(e) => {
            print_error(e);
        }
    };
}

pub fn append<T: IsFtpStream>(fs: &mut T, local_file: &mut File, remote_file: &str) {
    match fs.append_file(remote_file, local_file) {
        Ok(s) => {
            println!("{}", Color::White.paint(format!("{}", s)))
        }
        Err(e) => {
            print_error(e);
        }
    };
}

pub fn rm<T: IsFtpStream>(fs: &mut T, remote_file: &str) {
    match fs.rm(remote_file) {
        Ok(()) => {
            println!("{}", Color::Green.paint("[+] Success"));
        }
        Err(e) => {
            print_error(e);
        }
    };
}

pub fn rmdir<T: IsFtpStream>(fs: &mut T, remote_dir: &str) {
    match fs.rmdir(remote_dir) {
        Ok(()) => {
            println!("{}", Color::Green.paint("[+] Success"));
        }
        Err(e) => {
            print_error(e);
        }
    };
}

pub fn size<T: IsFtpStream>(fs: &mut T, remote_file: &str) {
    match fs.size(remote_file) {
        Ok(s) => {
            println!(
                "{}",
                format!(
                    "Size of {}: {}",
                    Color::Yellow.dimmed().paint(remote_file),
                    s
                )
            );
        }
        Err(e) => {
            print_error(e);
        }
    };
}

pub fn put<T: IsFtpStream>(fs: &mut T, local_file: &mut File, remote_file: &str) {
    match fs.put_file(remote_file, local_file) {
        Ok(n) => {
            println!("Wrote {} bytes", n);
        }
        Err(e) => {
            print_error(e);
        }
    };
}

pub fn get<T: IsFtpStream>(fs: &mut T, remote_file: &str, local_file: &mut File) {
    match fs.retr(remote_file, |stream| {
        let mut buf = Vec::new();
        stream
            .read_to_end(&mut buf)
            .map_err(|e| FtpError::ConnectionError(e))?;
        local_file
            .write(String::from_utf8(buf).unwrap().as_bytes())
            .map_err(|e| FtpError::ConnectionError(e))?;
        Ok(())
    }) {
        Ok(_) => {}
        Err(e) => {
            print_error(e);
        }
    };
}

pub fn user<T: IsFtpStream>(fs: &mut T, user: &str, pass: &str, cache: &mut String) {
    match fs.login(user, pass) {
        Ok(()) => {
            *cache = user.to_string();
        }
        Err(e) => {
            print_error(e);
        }
    };
}

pub fn quit<T: IsFtpStream>(fs: &mut T) {
    match fs.quit() {
        Ok(_) => {}
        Err(e) => {
            print_error(e);
        }
    };
}

pub fn noop<T: IsFtpStream>(fs: &mut T) {
    match fs.noop() {
        Ok(()) => {  }
        Err(e) => {
            print_error(e);
        }
    };
}

fn print_error(e: FtpError) {
    println!("{}", Color::Red.paint(format!("[-]: {}", e)));
}

pub fn help(cmd: &str) {
    if cmd == "" { return; }
    match cmd {
        "pwd" => {
            println!("USAGE:\n\t {}", Color::White.bold().paint("pwd")); 
            println!("Print the name of the current working directory on the remote machine");
        }
        "lpwd" => { 
            println!("USAGE:\n\t {}", Color::White.bold().paint("lpwd")); 
            println!("Print the working directory on the local machine");
        }
        "cd" | "cwd" => { 
            println!("USAGE:\n\t {} REMOTE_DIR", Color::White.bold().paint("cd|cwd")); 
            println!("Change the working directory on the remote machine to remote-directory");
        }
        "mkdir" => {
            println!("USAGE:\n\t {} REMOTE_DIR", Color::White.bold().paint("mkdir")); 
            println!("Make a directory on the remote machine");
        }
        "cdup" => { 
            println!("USAGE:\n\t {}", Color::White.bold().paint("cdup")); 
            println!("Move up one directory. This is equivalent to executing 'cd ..'");
        }
        "ls" | "dir" => {
            println!("USAGE:\n\t {} [REMOTE_DIR|REMOTE_FILE]", Color::White.bold().paint("ls|dir")); 
            println!("Print a listing of the contents of a directory on the remote machine. Regardless of the argument being a file or directory, human readable listing is printed");
        }
        "append" => {
            println!("USAGE:\n\t {} LOCAL_FILE REMOTE_FILE", Color::White.bold().paint("append")); 
            println!("Append a LOCAL_FILE a file on the remote machine.  If REMOTE_FILE is left unspecified, the LOCAL_FILE name is used in naming the remote file");
        }
        "delete" | "rm" => {
            println!("USAGE:\n\t {} REMOTE_FILES...", Color::White.bold().paint("delete|rm")); 
            println!("Delete files on the remote machine");
        }
        "rmdir" => {
            println!("USAGE:\n\t {} REMOTE_DIR", Color::White.bold().paint("rmdir")); 
            println!("Delete a directory on the remote machine");
        }
        "size" => { 
            println!("USAGE:\n\t {} REMOTE_FILES...", Color::White.bold().paint("size")); 
            println!("Return size of REMOTE_FILE on remote machine");
        }
        "get" => {
            println!("USAGE:\n\t {} REMOTE_FILE [LOCAL_FILE]", Color::White.bold().paint("get")); 
            println!("Retrieve the REMOTE_FILE and store it on the local machine.  If the LOCAL_FILE name is not specified, it is given the same name it has on the remote machine");
        }
        "put" => {
            println!("USAGE:\n\t {} LOCAL_FILE [REMOTE_FILE]", Color::White.bold().paint("put")); 
            println!("Store a LOCAL_FILE on the remote machine.  If REMOTE_FILE is left unspecified then the name of LOCAL_FILE is used");
        }
        "user" => {
            println!("USAGE:\n\t {} USER", Color::White.bold().paint("user")); 
            println!("Login as the specified USER");
        }
        "noop" => { 
            println!("USAGE:\n\t {}", Color::White.bold().paint("noop")); 
            println!("Does nothing, this is usually used to keep the FTP connection open");
        }
        "bye" | "quit" | "exit " => { 
            println!("USAGE:\n\t {}", Color::White.bold().paint("bye|quit|exit")); 
            println!("Closes off the FTP stream and exits the program")
        }
        "help" => { 
            println!("USAGE:\n\t {} [COMMAND]", Color::White.bold().paint("help"));
            print_available_commands();
        }
        _ => {
            println!("[-] Commnad: {cmd} not recognized");
            println!("Use the {} command for more information", Color::White.bold().paint("help"));
        }
    };
}

pub fn print_available_commands() {
    let commands = [ "cd|cwd", "put", "get", "pwd", "ls|dir", "bye|exit|quit", "noop", "user", "size", "mkdir", "rmdir",  "delete|rm", "append", "cdup", "lpwd", "help"];

    for (i, c) in commands.iter().enumerate() {
        if i%4 == 0 { println!(); }
        print!("{}\t", Color::White.bold().paint(*c));
        std::io::stdout().flush(); 
    }

    println!();
}
