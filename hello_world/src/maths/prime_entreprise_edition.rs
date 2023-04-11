pub struct PrimeNumber {
    value: i32,
}

impl PrimeNumber {
    const fn new() -> Self {
        Self { value: 1 }
    }
}

impl Default for PrimeNumber {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for PrimeNumber {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.value += 1;
            if is_prime(self.value) {
                return Some(self.value);
            }
        }
    }
}

fn is_prime(n: i32) -> bool {
    match n {
        1 => false,
        2 => true,
        _ => (2..=n / 2).all(|i| n % i != 0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_number() {
        let mut prime_number = PrimeNumber::default();
        assert_eq!(prime_number.next(), Some(2));
        assert_eq!(prime_number.next(), Some(3));
        assert_eq!(prime_number.next(), Some(5));
        assert_eq!(prime_number.next(), Some(7));
        assert_eq!(prime_number.next(), Some(11));
    }

    #[test]
    fn test_prime_number_function() {
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(5));
        assert!(is_prime(7));
        assert!(is_prime(11));
        assert!(!is_prime(1));
        assert!(!is_prime(4));
        assert!(!is_prime(6));
        assert!(!is_prime(8));
        assert!(!is_prime(9));
        assert!(!is_prime(10));
    }

    fn test_for_loop() {
        let vec_of_prime_numbers: Vec<_> = PrimeNumber::default().take(10).collect();
        for pin in vec_of_prime_numbers {
            assert!(is_prime(pin));
        }
    }
}