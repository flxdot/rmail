use rand::{thread_rng, Rng};
use rand::distributions::Distribution;
use clap::Parser;

const RANDOM_SIZE : usize = 6;

struct LowerAlphanumeric;

impl Distribution<u8> for LowerAlphanumeric {
   fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        const RANGE: usize = 26 + 10;
        const GEN_ASCII_STR_CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz\
                0123456789";
        let rand_idx: usize = rng.gen_range(0..RANGE);

        return GEN_ASCII_STR_CHARSET[rand_idx];
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

    let service = cli.service;
    let domain = "fanghanel.dev".to_string();

    let random = get_random();

    let email = format!("{}-{}@{}", service, random, domain);

    println!("{}", email);
}
  
