use crate::primes::is_prime;
use rand::Rng;

#[derive(Debug)]
pub struct KeyPair {
    private_key: Vec<u32>,
    a: u32,
    n: u32,
    public_key: Vec<u32>,
}

impl KeyPair {
    pub fn new(length: usize) -> Self {
        let (private_key, a, n) = generate_private_key(length);
        let public_key = generate_public_key(&private_key, a, n);

        Self {
            private_key,
            a,
            n,
            public_key,
        }
    }

    /// Get a reference to the key pair's private key.
    #[must_use]
    pub fn private_key(&self) -> &[u32] {
        self.private_key.as_ref()
    }

    /// Get the key pair's a.
    #[must_use]
    pub fn a(&self) -> u32 {
        self.a
    }

    /// Get the key pair's n.
    #[must_use]
    pub fn n(&self) -> u32 {
        self.n
    }

    /// Get a reference to the key pair's public key.
    #[must_use]
    pub fn public_key(&self) -> &[u32] {
        self.public_key.as_ref()
    }
}

fn is_superincreasing(sequence: &[u32]) -> bool {
    let mut sum = 0;

    for x in sequence.iter() {
        if *x < sum {
            return false;
        }
        sum += x;
    }

    true
}

fn generate_superincreasing_sequence(length: usize) -> Vec<u32> {
    let mut sequence: Vec<u32> = Vec::with_capacity(length);
    let start = rand::thread_rng().gen_range(2..10);
    sequence.push(start);
    let mut sum = start;

    for _ in 1..length {
        let new_element = sum + rand::thread_rng().gen_range(1..10);
        sequence.push(new_element);
        sum += new_element;
    }

    sequence
}

fn generate_private_key(length: usize) -> (Vec<u32>, u32, u32) {
    let sequence = generate_superincreasing_sequence(length);

    let sum: u32 = sequence.iter().sum();
    let mut n = sum;
    while !is_prime(n) {
        n += 1;
    }

    let a: u32 = rand::thread_rng().gen_range(2..1000);

    (sequence, a, n)
}

fn generate_public_key(private_key: &[u32], a: u32, n: u32) -> Vec<u32> {
    assert!(is_superincreasing(private_key));

    private_key.iter().map(|x| (x * a) % n).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_superincreasing() {
        let sequence = vec![2, 3, 6, 13, 27, 52, 105, 210];
        assert!(is_superincreasing(&sequence));
    }

    #[test]
    fn check_not_superincreasing() {
        let sequence = vec![1, 3, 4, 9, 15, 25, 48, 76];
        assert!(!is_superincreasing(&sequence));
    }

    #[test]
    fn generate_sequence() {
        let sequence = generate_superincreasing_sequence(8);
        assert!(is_superincreasing(&sequence));
    }

    #[test]
    fn public_key_from_private() {
        let private_key = vec![2, 3, 6, 13, 27, 52, 105, 210];
        let expected = vec![62, 93, 186, 403, 417, 352, 315, 210];

        assert_eq!(generate_public_key(&private_key, 31, 420), expected);
    }
}
