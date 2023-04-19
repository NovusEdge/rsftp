use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Address of the host to connect to
    #[arg(name = "host")]
    pub host: String,

    /// Port on which connection should be established
    #[arg(short, long, default_value_t = 21)]
    pub port: u16,

    /// Use FTPS through TLS
    #[arg(long, default_value_t = false)]
    pub tls: bool,

    /// Username to use for login
    #[arg(short='U', long, default_value_t = String::from("anonymous"))]
    pub username: String,

    /// Password to use for login
    #[arg(short='P', long, default_value_t = String::from(""))]
    pub password: String,
}

pub fn init() -> Args {
    Args::parse()
}
