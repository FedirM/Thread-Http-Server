use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    /// Target directory to serve from local machine
    dir: Option<PathBuf>,

    /// Set a custom port for the server
    #[arg(short, long, value_name = "PORT")]
    port: Option<u16>,
}

pub fn parse_cli() -> (u16, PathBuf) {
    let mut port = 7878;
    let mut host_dir = std::env::current_dir().expect("Could not get pwd!");

    let cli = Cli::parse();

    if let Some(p) = cli.port {
        port = p;
    }

    if let Some(dir) = cli.dir {
        if dir.exists() {
            if dir.is_dir() {
                host_dir = dir;
            } else {
                panic!("Setted path is not a directory!")
            }
        } else {
            panic!("Setted path `{}` does not exists!", dir.to_str().unwrap());
        }
    }

    (port, host_dir)
}
