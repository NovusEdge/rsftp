mod cli;
mod ftp_handler;

fn main() {
    let args = cli::parsing::init();
    println!("{:#?}", args);

    if !args.tls {
        let host = args.host.as_str();
        let mut current_user = args.username;
        let mut fs = ftp_handler::client::connect(
            host.to_string(),
            args.port,
            current_user.as_str(),
            args.password.as_str(),
        );

        loop {
            let pwd = fs.pwd().unwrap_or("/".to_string());
            current_user =
                cli::prompt::prompt_user(pwd.as_str(), host, current_user.as_str(), &mut fs);
        }
    } else {
        let host = args.host.as_str();
        let mut current_user = args.username;
        let mut fs = ftp_handler::client::connect_tls(
            host.to_string(),
            args.port,
            current_user.as_str(),
            args.password.as_str(),
        );

        loop {
            let pwd = fs.pwd().unwrap_or("/".to_string());
            current_user =
                cli::prompt::prompt_user(pwd.as_str(), host, current_user.as_str(), &mut fs);
        }
    }
}
