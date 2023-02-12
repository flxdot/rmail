use clap::{ArgGroup, Parser};
use config::ConfigFile;
use rand::distributions::Distribution;
use rand::{thread_rng, Rng};
use std::io::{self, Write};

mod config;

const RANDOM_SIZE: usize = 6;

struct LowerAlphanumeric;

impl Distribution<u8> for LowerAlphanumeric {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        const CHOICES: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
        let idx: usize = rng.gen_range(0..CHOICES.len());
        return CHOICES[idx];
    }
}

fn get_random() -> String {
    return thread_rng()
        .sample_iter(&LowerAlphanumeric)
        .take(RANDOM_SIZE)
        .map(char::from)
        .collect();
}

fn ask_user_for_input(name: String, phrase: Option<String>) -> String {
    let mut input = String::new();
    let phrase = format!(
        "{}",
        phrase.unwrap_or(format!("What is the desired {}?: ", name))
    );

    print!("{}", &phrase);
    let _ = io::stdout().flush();

    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => break,
            Err(_) => {
                let _ = io::stdout().flush();
                print!("{}", &phrase)
            }
        }
    }

    return input.trim().to_string();
}

fn run_init() {
    let domain = ask_user_for_input("domain".to_string(), None);
    if domain.contains("@") && domain.contains(".") {
        println!(
            "{} seems not to be a valid domain. Remember not to include the @ sign.",
            domain
        );
    }
    let new_config = config::Config { domain: domain };

    new_config.save();
}

fn generate_email(service: &str) -> String {
    let config = config::Config::load();
    let domain = config.domain;

    let email = format!("{}-{}@{}", service, get_random(), domain);
    return email;
}

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
        Cli { init: true, .. } => run_init(),
        Cli { service: Some(service), .. } => println!("{}", generate_email(&service.to_string())),
        _ => unreachable!(),
    };
}
