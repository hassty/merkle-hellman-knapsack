use std::{
    error::Error,
    fmt::Debug,
    fs::File,
    io::{self, BufRead, BufReader, Read, Write},
    path::{Path, PathBuf},
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
    Encrypt {
        /// Public key of the recipient
        #[clap(short, long, default_value = "public.key", parse(from_os_str))]
        keyfile: PathBuf,
        /// Message to encrypt
        #[clap(required = false, parse(from_os_str))]
        textfile: Option<PathBuf>,
    },
    /// Decrypt the cipher
    Decrypt {
        /// Private key of the recipient
        #[clap(short, long, default_value = "private.key", parse(from_os_str))]
        keyfile: PathBuf,
        /// Encrypted message
        #[clap(required = false, parse(from_os_str))]
        cipherfile: Option<PathBuf>,
    },
}

fn generate_keys(private_key: &Path, public_key: &Path) -> io::Result<()> {
    let keys = keys::KeyPair::new(8);

    let mut private_key_file = File::create(&private_key)?;
    private_key_file.write_fmt(format_args!(
        "{}\n{}\n{}",
        keys.private_key()
            .iter()
            .map(|x| x.to_string() + " ")
            .collect::<String>(),
        keys.a(),
        keys.n()
    ))?;

    let mut public_key_file = File::create(&public_key)?;

    public_key_file.write_fmt(format_args!(
        "{}",
        keys.public_key()
            .iter()
            .map(|x| x.to_string() + " ")
            .collect::<String>()
    ))?;

    Ok(())
}

fn encrypt(keyfile: PathBuf, textfile: Option<PathBuf>) -> Result<String, Box<dyn Error>> {
    let input = File::open(keyfile)?;
    let buffered = BufReader::new(input);
    let public_key = buffered
        .lines()
        .last()
        .ok_or("unable to read public key")??;
    let public_key = public_key
        .split_whitespace()
        .map(|x| x.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut input: Box<dyn io::Read> = if let Some(textfile) = textfile {
        Box::new(File::open(textfile)?)
    } else {
        Box::new(io::stdin())
    };

    let mut text = String::new();
    input.read_to_string(&mut text)?;
    let encrypted = merkle_hellman_knapsack::encrypt(text.trim(), &public_key);

    Ok(encrypted
        .iter()
        .map(|x| x.to_string() + " ")
        .collect::<String>())
}

fn decrypt(keyfile: PathBuf, cipherfile: Option<PathBuf>) -> Result<String, Box<dyn Error>> {
    let input = File::open(keyfile)?;
    let mut lines = BufReader::new(input).lines();
    let private_key = lines
        .next()
        .ok_or("unable to read private key")??
        .split_whitespace()
        .map(|x| x.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?;
    let a = lines
        .next()
        .ok_or("invalid parameter 'a'")??
        .parse::<u32>()?;
    let n = lines
        .next()
        .ok_or("invalid parameter 'n'")??
        .parse::<u32>()?;

    let mut input: Box<dyn io::Read> = if let Some(cipherfile) = cipherfile {
        Box::new(File::open(cipherfile)?)
    } else {
        Box::new(io::stdin())
    };

    let mut text = String::new();
    input.read_to_string(&mut text)?;

    let cipher = text
        .split_whitespace()
        .map(|x| x.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?;

    let decrypted = merkle_hellman_knapsack::decrypt(&cipher, &private_key, a, n);
    Ok(decrypted)
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Keys {
            private_key,
            public_key,
        } => match generate_keys(&private_key, &public_key) {
            Ok(_) => {
                println!("saved private key to {}", private_key.to_str().unwrap());
                println!("saved public key to {}", public_key.to_str().unwrap());
            }
            Err(error) => {
                eprintln!("{}", error);
            }
        },
        Commands::Encrypt { keyfile, textfile } => match encrypt(keyfile, textfile) {
            Ok(cipher) => {
                println!("{}", cipher);
            }
            Err(error) => {
                eprintln!("{}", error);
            }
        },
        Commands::Decrypt {
            keyfile,
            cipherfile,
        } => match decrypt(keyfile, cipherfile) {
            Ok(decrypted) => {
                println!("{}", decrypted);
            }
            Err(error) => {
                eprintln!("{}", error);
            }
        },
    }
}
