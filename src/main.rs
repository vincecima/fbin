use clap::Command;
use reqwest::{Error, Response};

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
const ABOUT: Option<&str> = option_env!("CARGO_PKG_DESCRIPTION");

fn main() -> Result<(), Error> {
    let matches = Command::new("fbin")
        .version(VERSION.unwrap_or("unknown"))
        .about(ABOUT.unwrap_or("unknown"))
        .subcommand(Command::new("pages").subcommand(Command::new("create")))
        .get_matches();

    match matches.subcommand() {
        Some(("pages", sub_matches)) => match sub_matches.subcommand() {
            Some(("create", _sub_matches)) => {
                let mut res = reqwest::blocking::get("https://api.feedbin.com/v2/")?;
                // eprintln!("Response: {:?} {}", res.version(), res.status());
                // eprintln!("Headers: {:#?}\n", res.headers());
                // res.copy_to(&mut std::io::stdout())?;
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}
