use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Address of the host to connect to
    #[arg(name = "host")]
    host: String,

    /// Port on which connection should be established
    #[arg(short, long, default_value_t = 21)]
    port: u8,

    /// Use FTPS through openssl
    #[arg(long, default_value_t = false)]
    openssl: bool,
}

pub fn init() -> Args {
    Args::parse()
}
