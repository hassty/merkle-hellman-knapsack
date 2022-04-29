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
