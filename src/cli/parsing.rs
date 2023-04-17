use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Address of the host to connect to
    #[arg(name = "host")]
    host: String,

    /// Port on which connection should be established
    #[arg(short, long, default_value_t = 21)]
    port: u16,

    /// Use FTPS through openssl
    #[arg(long, default_value_t = false)]
    openssl: bool,

    /// Username to use for login
    #[arg(short='U', long, default_value_t = String::from("anonymous"))]
    username: String,

    /// Password to use for login
    #[arg(short='P', long, default_value_t = String::from(""))]
    password: String,
}

pub fn init() -> Args {
    Args::parse()
}
