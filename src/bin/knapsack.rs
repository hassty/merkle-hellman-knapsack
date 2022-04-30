use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

use clap::{Parser, Subcommand};
use lab9::{keys, merkle_hellman_knapsack};

#[derive(Parser)]
#[clap(name = "knapsack")]
#[clap(about="merkle-hellman knapsack cryptosystem", long_about=None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[clap(arg_required_else_help = false)]
    /// Generate private and public keys and save them to file
    Keys {
        /// Path to private key file
        #[clap(long, default_value = "private.key", parse(from_os_str))]
        private_key: PathBuf,
        /// Path to pulic key file
        #[clap(long, default_value = "public.key", parse(from_os_str))]
        public_key: PathBuf,
    },
    /// Encrypt given text
    #[clap(arg_required_else_help = true)]
    Encrypt {
        /// Public key of the recipient
        #[clap(short, long, default_value = "public.key", parse(from_os_str))]
        keyfile: PathBuf,
        /// Message to encrypt
        #[clap(required = true)]
        text: String,
    },
    /// Decrypt the cipher
    #[clap(arg_required_else_help = true)]
    Decrypt {
        /// Private key of the recipient
        #[clap(short, long, default_value = "private.key", parse(from_os_str))]
        keyfile: PathBuf,
        /// Encrypted message
        #[clap(multiple_values = true, required = true)]
        cipher: Vec<u32>,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Keys {
            private_key,
            public_key,
        } => {
            let keys = keys::KeyPair::new(8);

            let mut private_key_file =
                File::create(&private_key).expect("unable to create file {private_key}");
            private_key_file
                .write_fmt(format_args!(
                    "{}\n{}\n{}",
                    keys.private_key()
                        .iter()
                        .map(|x| x.to_string() + " ")
                        .collect::<String>(),
                    keys.a(),
                    keys.n()
                ))
                .expect("unable to write private key to {private_key}");
            println!("saved private key to {}", private_key.to_str().unwrap(),);

            let mut public_key_file =
                File::create(&public_key).expect("unable to create file {public_key}");

            public_key_file
                .write_fmt(format_args!(
                    "{}",
                    keys.public_key()
                        .iter()
                        .map(|x| x.to_string() + " ")
                        .collect::<String>()
                ))
                .expect("unable to write public key to {public_key}");
            println!("saved public key to {}", public_key.to_str().unwrap(),);
        }
        Commands::Encrypt { keyfile, text } => {
            let input = File::open(keyfile).expect("invalid path to keyfile");
            let buffered = BufReader::new(input);
            let public_key = buffered
                .lines()
                .last()
                .expect("missing public key")
                .unwrap();
            let public_key = public_key
                .split_whitespace()
                .map(|x| x.parse::<u32>().expect("invalid public key"))
                .collect::<Vec<u32>>();
            let encrypted = merkle_hellman_knapsack::encrypt(&text, &public_key);

            println!(
                "{}",
                encrypted
                    .iter()
                    .map(|x| x.to_string() + " ")
                    .collect::<String>()
            );
        }
        Commands::Decrypt { keyfile, cipher } => {
            let input = File::open(keyfile).expect("invalid path to keyfile");
            let mut lines = BufReader::new(input).lines();
            let private_key = lines
                .next()
                .expect("missing private key")
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<u32>().expect("invalid private key"))
                .collect::<Vec<u32>>();
            let a = lines
                .next()
                .expect("missing parameter 'a'")
                .unwrap()
                .parse::<u32>()
                .expect("invalid parameter 'a'");
            let n = lines
                .next()
                .expect("missing parameter 'n'")
                .unwrap()
                .parse::<u32>()
                .expect("invalid parameter 'n'");
            let decrypted = merkle_hellman_knapsack::decrypt(&cipher, &private_key, a, n);

            println!("{decrypted}");
        }
    }
}
