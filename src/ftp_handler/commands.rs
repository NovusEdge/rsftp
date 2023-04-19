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

pub fn chmod<T: IsFtpStream>(fs: &mut T, remote_file: &str) {
    unimplemented!()
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

pub fn help() {
    todo!()
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
