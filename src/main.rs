mod cli;
mod ftp_handler;

fn main() {
    let args = cli::parsing::init();
    println!("{:#?}", args);

}
