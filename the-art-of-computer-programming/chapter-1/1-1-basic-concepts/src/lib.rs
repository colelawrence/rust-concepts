
/// Page 2. Algorithm E (Euclid's algorithm)
/// Given two positive integers, m and n,
/// find their greatest common divisor, that is,
/// the largest positive integer which evenly
/// divides both m and n.
pub fn gcd(m: usize, n: usize) -> usize {
    let r = m % n;
    if r == 0 {
        n
    } else {
        gcd(n, r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_works() {
        assert_eq!(gcd(4, 4), 4);
        assert_eq!(gcd(8, 4), 4);
        assert_eq!(gcd(8, 2), 2);
        assert_eq!(gcd(132, 2), 2);
        assert_eq!(gcd(2, 132), 2);
        assert_eq!(gcd(9, 132), 3);
        assert_eq!(gcd(19823785, 289374), 17);
    }
}
