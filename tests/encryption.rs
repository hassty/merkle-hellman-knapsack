use lab9::keys;
use lab9::merkle_hellman_knapsack;

#[test]
fn decrypt_word() {
    let message = "word";
    let keys = keys::KeyPair::new(8);

    let encrypted = merkle_hellman_knapsack::encrypt(message, keys.public_key());
    let decrypted =
        merkle_hellman_knapsack::decrypt(&encrypted, keys.private_key(), keys.a(), keys.n());

    assert_eq!(decrypted, message);
}

#[test]
fn decrypt_phrase() {
    let message = "attack at dawn";
    let keys = keys::KeyPair::new(8);

    let encrypted = merkle_hellman_knapsack::encrypt(message, keys.public_key());
    let decrypted =
        merkle_hellman_knapsack::decrypt(&encrypted, keys.private_key(), keys.a(), keys.n());

    assert_eq!(decrypted, message);
}

#[test]
fn decrypt_uppercase() {
    let message = "ATTACK AT DAWN";
    let keys = keys::KeyPair::new(8);

    let encrypted = merkle_hellman_knapsack::encrypt(message, keys.public_key());
    let decrypted =
        merkle_hellman_knapsack::decrypt(&encrypted, keys.private_key(), keys.a(), keys.n());

    assert_eq!(decrypted, message);
}
