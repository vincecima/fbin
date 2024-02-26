use clap::Command;
const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
const ABOUT: Option<&str> = option_env!("CARGO_PKG_DESCRIPTION");

fn main() {
    let _matches = Command::new("fbin")
        .version(VERSION.unwrap_or("unknown"))
        .about(ABOUT.unwrap_or("unknown"))
        .subcommand(Command::new("pages").subcommand(Command::new("create")))
        .get_matches();
}
