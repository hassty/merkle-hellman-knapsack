mod primes {
    pub fn modinv(a: u32, n: u32) -> u32 {
        let a = a as i32;
        let n = n as i32;
        let mut s: i32 = 0;
        let mut r: i32 = n as i32;
        let mut old_s: i32 = 1;
        let mut old_r: i32 = a as i32;

        while r != 0 {
            let quotient = old_r / r;
            (old_r, r) = (r, old_r - quotient * r);
            (old_s, s) = (s, old_s - quotient * s);
        }

        while old_s < 0 {
            old_s = n - old_s.abs();
        }

        old_s as u32
    }

    pub fn is_prime(n: u32) -> bool {
        for i in 2..n {
            if n % i == 0 {
                return false;
            }
        }

        true
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn modular_inverse() {
            let a = 588;
            let n = 881;

            assert_eq!(modinv(a, n), 442);
        }

        #[test]
        fn prime() {
            assert!(is_prime(607));
        }

        #[test]
        fn not_prime() {
            assert!(!is_prime(420));
        }
    }
}

pub mod keys {
    use crate::primes::is_prime;
    use rand::Rng;

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

    pub fn generate_keys(length: usize) -> (Vec<u32>, u32, u32, Vec<u32>) {
        let (private_key, a, n) = generate_private_key(length);
        let public_key = generate_public_key(&private_key, a, n);

        (private_key, a, n, public_key)
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
}

pub mod merkle_hellman_knapsack {
    use crate::primes::modinv;

    pub fn encrypt(message: &str, public_key: &[u32]) -> Vec<u32> {
        message
            .chars()
            .map(|c| {
                let mut result = 0;
                let binary = format!("{:08b}", u32::from(c));

                for (i, c) in binary.chars().enumerate() {
                    result += c.to_digit(2).unwrap() * public_key[i];
                }

                result
            })
            .collect()
    }

    pub fn decrypt(cipher: &[u32], private_key: &[u32], a: u32, n: u32) -> String {
        let mut result = String::new();
        let modinverse = modinv(a, n);

        for c in cipher.iter() {
            let mut cc = ((c * modinverse) % n) as i32;
            let mut x: Vec<u32> = Vec::with_capacity(8);
            for p in private_key.iter().rev() {
                let p = *p as i32;
                if cc - p >= 0 {
                    cc -= p;
                    x.push(1);
                } else {
                    x.push(0);
                }
            }

            let x = x.iter().rev().map(|c| c.to_string()).collect::<String>();
            let x = u32::from_str_radix(&x, 2).unwrap();
            result.push(char::from_u32(x).unwrap())
        }

        result
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn encryption() {
            let public_key = vec![295, 592, 301, 14, 28, 353, 120, 236];
            let message = "a";

            let cipher = encrypt(message, &public_key);
            assert_eq!(cipher, vec![1129]);
        }

        #[test]
        fn decryption() {
            let private_key = vec![2, 7, 11, 21, 42, 89, 180, 354];
            let a = 588;
            let n = 881;
            let cipher = vec![1129];

            let message = decrypt(&cipher, &private_key, a, n);
            assert_eq!(message, "a");
        }
    }
}
