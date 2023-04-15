use suppaftp::native_tls::{TlsConnector, TlsStream};
use suppaftp::FtpStream;
use suppaftp::{NativeTlsConnector, NativeTlsFtpStream};

fn connect(host: String, port: u8, username: &str, password: &str) -> FtpStream {
    #[allow(unused_mut)]
    let mut ftp_stream = FtpStream::connect(format!("{}:{}", host, port)).unwrap();

    let _ = ftp_stream.login(username, password).unwrap();

    ftp_stream
}

fn connect_tls(host: String, port: u8, username: &str, password: &str) -> NativeTlsFtpStream {
    #[allow(unused_mut)]
    let mut ftp_stream = NativeTlsFtpStream::connect(format!("{}:{}", host, port)).unwrap();
    // Switch to the secure mode
    let mut ftp_stream = ftp_stream
        .into_secure(
            NativeTlsConnector::from(TlsConnector::new().unwrap()),
            host.as_str(),
        )
        .unwrap();
    ftp_stream.login(username, password).unwrap();

    ftp_stream
}
