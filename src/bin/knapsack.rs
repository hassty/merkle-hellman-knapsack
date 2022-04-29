use lab9::{keys, merkle_hellman_knapsack};

fn main() {
    let message = "ttm";
    let (private_key, a, n, public_key) = keys::generate_keys(8);

    let cipher = merkle_hellman_knapsack::encrypt(message, &public_key);
    println!("cipher: {cipher:?}");

    let decrypted = merkle_hellman_knapsack::decrypt(&cipher, &private_key, a, n);
    println!("decrypted: {decrypted}");
}
