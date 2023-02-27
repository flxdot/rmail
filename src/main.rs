use clap::{ArgGroup, Parser};
use rmail::{build_config, generate_email};

/// Simple program to generate random Email addresses
#[derive(Parser, Debug)]
#[command(name = "rmail")]
#[command(about, long_about = None)]
#[command(group(
            ArgGroup::new("cli")
                    .required(true)
                    .args(["service", "init"]),
        ))]
struct Cli {
    /// the service the email shall be used for
    #[arg()]
    service: Option<String>,
    /// A flag to indicate that you want to initialize the config
    #[arg(short, long)]
    init: bool,
}

fn main() {
    let cli = Cli::parse();

    match cli {
        Cli { init: true, .. } => build_config(),
        Cli {
            service: Some(service),
            ..
        } => println!("{}", generate_email(&service.to_string())),
        _ => unreachable!(),
    };
}
