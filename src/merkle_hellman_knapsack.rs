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
