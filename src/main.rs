mod cli;
mod ftp_handler;


fn main() {
    let args = cli::init();
    println!("{:#?}", args);
}
