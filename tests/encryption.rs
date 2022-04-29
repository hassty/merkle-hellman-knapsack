use lab9::keys;
use lab9::merkle_hellman_knapsack;

#[test]
fn decrypt_word() {
    let message = "word";
    let (private_key, a, n, public_key) = keys::generate_keys(8);

    let encrypted = merkle_hellman_knapsack::encrypt(message, &public_key);
    let decrypted = merkle_hellman_knapsack::decrypt(&encrypted, &private_key, a, n);

    assert_eq!(decrypted, message);
}

#[test]
fn decrypt_phrase() {
    let message = "attack at dawn";
    let (private_key, a, n, public_key) = keys::generate_keys(8);

    let encrypted = merkle_hellman_knapsack::encrypt(message, &public_key);
    let decrypted = merkle_hellman_knapsack::decrypt(&encrypted, &private_key, a, n);

    assert_eq!(decrypted, message);
}

#[test]
fn decrypt_uppercase() {
    let message = "ATTACK AT DAWN";
    let (private_key, a, n, public_key) = keys::generate_keys(8);

    let encrypted = merkle_hellman_knapsack::encrypt(message, &public_key);
    let decrypted = merkle_hellman_knapsack::decrypt(&encrypted, &private_key, a, n);

    assert_eq!(decrypted, message);
}
