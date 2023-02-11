use clap::Parser;
use rand::distributions::Distribution;
use rand::{thread_rng, Rng};

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

/// Simple program to generate random Email addresses
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Cli {
    /// the service the email shall be used for
    #[arg()]
    service: String,
}

fn main() {
    let cli = Cli::parse();

    let domain = "fanghanel.dev".to_string();

    let email = format!("{}-{}@{}", cli.service, get_random(), domain);

    println!("{}", email);
}
