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
    /// Generate private and public keys
    Keys,
    /// Encrypt given text
    #[clap(arg_required_else_help = true)]
    Encrypt {
        /// Public key of the recipient
        #[clap(multiple_values = true, required = true)]
        public_key: Vec<u32>,
        /// Message to encrypt
        #[clap(required = true)]
        text: String,
    },
    /// Decrypt the cipher
    Decrypt {
        /// Private key of the recipient
        #[clap(multiple_values = true, required = true)]
        private_key: Vec<u32>,
        #[clap(short)]
        a: u32,
        #[clap(short)]
        n: u32,
        /// Encrypted message
        #[clap(multiple_values = true, required = true, last = true)]
        cipher: Vec<u32>,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Keys => {
            let (private_key, a, n, public_key) = keys::generate_keys(8);

            println!(
                "{}",
                private_key
                    .iter()
                    .map(|x| x.to_string() + " ")
                    .collect::<String>()
            );
            println!("{a} {n}");
            println!(
                "{}",
                public_key
                    .iter()
                    .map(|x| x.to_string() + " ")
                    .collect::<String>()
            );
        }
        Commands::Encrypt { public_key, text } => {
            let encrypted = merkle_hellman_knapsack::encrypt(&text, &public_key);

            println!(
                "{}",
                encrypted
                    .iter()
                    .map(|x| x.to_string() + " ")
                    .collect::<String>()
            );
        }
        Commands::Decrypt {
            private_key,
            a,
            n,
            cipher,
        } => {
            let decrypted = merkle_hellman_knapsack::decrypt(&cipher, &private_key, a, n);

            println!("{decrypted}");
        }
    }
}
