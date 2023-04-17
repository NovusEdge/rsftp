use std::fs::File;
use ansi_term::Color;
use suppaftp::{types::FileType, FtpError, FtpStream, NativeTlsFtpStream};

pub fn pwd(fs: &mut FtpStream) {
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

pub fn cwd(fs: &mut FtpStream, remote_dir: &str) {
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

pub fn cd(fs: &mut FtpStream, remote_dir: &str) {
    cwd(fs, remote_dir);
}

pub fn cdup(fs: &mut FtpStream) {
    cwd(fs, "..");
}

pub fn ls(fs: &mut FtpStream, remote_dir: &str) {
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

pub fn dir(fs: &mut FtpStream, remote_dir: &str) {
    ls(fs, remote_dir);
}

pub fn mkdir(fs: &mut FtpStream, new_dir: &str) {
    match fs.mkdir(new_dir) {
        Ok(()) => {
            println!("{}", Color::Green.paint("[+]: Success!"));
        }
        Err(e) => {
            print_error(e);
        }
    };
}

pub fn append(fs: &mut FtpStream, local_file: &mut File, remote_file: &str) {
    match fs.append_file(remote_file, local_file) {
        Ok(s) => {
            println!("{}", Color::White.paint(format!("{}", s)))
        }
        Err(e) => {
            print_error(e);
        }
    };
}

pub fn rm(fs: &mut FtpStream, remote_file: &str) {
    match fs.rm(remote_file) {
        Ok(()) => {
            println!("{}", Color::Green.paint("[+] Success"));
        }
        Err(e) => {
            print_error(e);
        }
    };
}

pub fn delete(fs: &mut FtpStream, remote_file: &str) {
    rm(fs, remote_file);
}

pub fn rmdir(fs: &mut FtpStream, remote_dir: &str) {
    match fs.rmdir(remote_dir) {
        Ok(()) => {
            println!("{}", Color::Green.paint("[+] Success"));
        }
        Err(e) => {
            print_error(e);
        }
    };

}

pub fn size(fs: &mut FtpStream, remote_file: &str) {
    match fs.size(remote_file) {
        Ok(s) => {
            println!("{}", format!("Size of {}: {}", Color::Yellow.dimmed().paint(remote_file), s));
        }
        Err(e) => {
            print_error(e);
        }
    };
}

pub fn chmod(fs: &mut FtpStream, remote_file: &str) {
    unimplemented!()
}

pub fn put(fs: &mut FtpStream, local_file: &mut File, remote_file: &str) {
    match fs.put_file(remote_file, local_file) {
        Ok(n) => { println!("Wrote {} bytes", n); }
        Err(e) => { print_error(e); }
    };
}

fn print_error(e: FtpError) {
    println!("{}", Color::Red.paint(format!("[-]: {}", e)));
}

// Functions for TLS stream:
