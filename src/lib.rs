use rand::distributions::Distribution;
use rand::{thread_rng, Rng};
mod config;
pub use config::{build_config, ConfigFile};

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

pub fn generate_email(service: &str) -> String {
    let config = config::Config::load();
    let domain = config.domain;

    let email = format!("{}-{}@{}", service, get_random(), domain);
    return email;
}
