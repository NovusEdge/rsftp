use std::io::Read;
use suppaftp::native_tls::TlsConnector;
use suppaftp::FtpStream;
use suppaftp::{FtpResult, NativeTlsConnector, NativeTlsFtpStream};

pub fn connect(host: String, port: u16, username: &str, password: &str) -> FtpStream {
    #[allow(unused_mut)]
    let mut ftp_stream = FtpStream::connect(format!("{}:{}", host, port)).unwrap();
    let _ = ftp_stream.login(username, password).unwrap();

    ftp_stream
}

pub fn connect_tls(host: String, port: u16, username: &str, password: &str) -> NativeTlsFtpStream {
    #[allow(unused_mut)]
    let mut ftp_stream = NativeTlsFtpStream::connect(format!("{}:{}", host, port)).unwrap();
    let mut ftp_stream = ftp_stream
        .into_secure(
            NativeTlsConnector::from(TlsConnector::new().unwrap()),
            host.as_str(),
        )
        .unwrap();
    ftp_stream.login(username, password).unwrap();

    ftp_stream
}

pub trait IsFtpStream {
    fn pwd(&mut self) -> FtpResult<String>;
    fn cdup(&mut self) -> FtpResult<()>;
    fn quit(&mut self) -> FtpResult<()>;
    fn noop(&mut self) -> FtpResult<()>;
    fn cwd<S: AsRef<str>>(&mut self, path: S) -> FtpResult<()>;
    fn mkdir<S: AsRef<str>>(&mut self, pathname: S) -> FtpResult<()>;
    fn list(&mut self, pathname: Option<&str>) -> FtpResult<Vec<String>>;
    fn size<S: AsRef<str>>(&mut self, pathname: S) -> FtpResult<usize>;
    fn append_file<R: Read>(&mut self, filename: &str, r: &mut R) -> FtpResult<u64>;

    fn rm<S: AsRef<str>>(&mut self, filename: S) -> FtpResult<()>;
    fn rmdir<S: AsRef<str>>(&mut self, pathname: S) -> FtpResult<()>;
    fn retr<F, D>(&mut self, file_name: &str, reader: F) -> FtpResult<D>
    where
        F: FnMut(&mut dyn Read) -> FtpResult<D>;
    fn put_file<S: AsRef<str>, R: Read>(&mut self, filename: S, r: &mut R) -> FtpResult<u64>;
    fn login<S: AsRef<str>>(&mut self, user: S, password: S) -> FtpResult<()>;
}

impl IsFtpStream for FtpStream {
    fn pwd(&mut self) -> FtpResult<String> {
        self.pwd()
    }
    fn cdup(&mut self) -> FtpResult<()> {
        self.cdup()
    }
    fn quit(&mut self) -> FtpResult<()> {
        self.quit()
    }
    fn noop(&mut self) -> FtpResult<()> {
        self.noop()
    }
    fn cwd<S: AsRef<str>>(&mut self, path: S) -> FtpResult<()> {
        self.cwd(path)
    }
    fn mkdir<S: AsRef<str>>(&mut self, pathname: S) -> FtpResult<()> {
        self.mkdir(pathname)
    }
    fn list(&mut self, pathname: Option<&str>) -> FtpResult<Vec<String>> {
        self.list(pathname)
    }

    fn size<S: AsRef<str>>(&mut self, pathname: S) -> FtpResult<usize> {
        self.size(pathname)
    }
    fn append_file<R: Read>(&mut self, filename: &str, r: &mut R) -> FtpResult<u64> {
        self.append_file(filename, r)
    }

    fn rm<S: AsRef<str>>(&mut self, filename: S) -> FtpResult<()> {
        self.rm(filename)
    }
    fn rmdir<S: AsRef<str>>(&mut self, pathname: S) -> FtpResult<()> {
        self.rmdir(pathname)
    }
    fn retr<F, D>(&mut self, file_name: &str, reader: F) -> FtpResult<D>
    where
        F: FnMut(&mut dyn Read) -> FtpResult<D>,
    {
        self.retr(file_name, reader)
    }
    fn put_file<S: AsRef<str>, R: Read>(&mut self, filename: S, r: &mut R) -> FtpResult<u64> {
        self.put_file(filename, r)
    }

    fn login<S: AsRef<str>>(&mut self, user: S, password: S) -> FtpResult<()> {
        self.login(user, password)
    }
}

impl IsFtpStream for NativeTlsFtpStream {
    fn pwd(&mut self) -> FtpResult<String> {
        self.pwd()
    }
    fn cdup(&mut self) -> FtpResult<()> {
        self.cdup()
    }
    fn quit(&mut self) -> FtpResult<()> {
        self.quit()
    }
    fn noop(&mut self) -> FtpResult<()> {
        self.noop()
    }
    fn cwd<S: AsRef<str>>(&mut self, path: S) -> FtpResult<()> {
        self.cwd(path)
    }
    fn mkdir<S: AsRef<str>>(&mut self, pathname: S) -> FtpResult<()> {
        self.mkdir(pathname)
    }
    fn list(&mut self, pathname: Option<&str>) -> FtpResult<Vec<String>> {
        self.list(pathname)
    }
    fn size<S: AsRef<str>>(&mut self, pathname: S) -> FtpResult<usize> {
        self.size(pathname)
    }
    fn append_file<R: Read>(&mut self, filename: &str, r: &mut R) -> FtpResult<u64> {
        self.append_file(filename, r)
    }

    fn rm<S: AsRef<str>>(&mut self, filename: S) -> FtpResult<()> {
        self.rm(filename)
    }
    fn rmdir<S: AsRef<str>>(&mut self, pathname: S) -> FtpResult<()> {
        self.rmdir(pathname)
    }
    fn retr<F, D>(&mut self, file_name: &str, reader: F) -> FtpResult<D>
    where
        F: FnMut(&mut dyn Read) -> FtpResult<D>,
    {
        self.retr(file_name, reader)
    }

    fn put_file<S: AsRef<str>, R: Read>(&mut self, filename: S, r: &mut R) -> FtpResult<u64> {
        self.put_file(filename, r)
    }

    fn login<S: AsRef<str>>(&mut self, user: S, password: S) -> FtpResult<()> {
        self.login(user, password)
    }
}
