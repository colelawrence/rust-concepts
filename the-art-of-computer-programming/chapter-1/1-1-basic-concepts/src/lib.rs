#[macro_use] extern crate log;

/// Page 2. Algorithm E (Euclid's algorithm)
/// Given two positive integers, m and n,
/// find their greatest common divisor, that is,
/// the largest positive integer which evenly
/// divides both m and n.
pub fn gcd(m: usize, n: usize) -> usize {
    let r = m % n;
    debug!("r={} \tof \tm={} \tand n={}", r, m, n);
    if r == 0 {
        n
    } else {
        gcd(n, r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use env_logger;

    fn init_logger(context: &str) {
        use env_logger::Builder;
        Builder::from_default_env()
            .default_format_timestamp(false)
            .default_format_level(false)
            .init();
        let _ = env_logger::try_init();
        info!("{}", context);
    }

    fn gcd_test(m: usize, n: usize) -> usize {
        debug!("INPUT\tm={},\tn={}", m, n);
        let result = gcd(m, n);
        debug!("RESULT\t= {}", result);
        result
    }

    #[test]
    fn gcd_works() {
        init_logger("Testing GCD");
        assert_eq!(gcd_test(4, 4), 4);
        assert_eq!(gcd_test(8, 4), 4);
        assert_eq!(gcd_test(8, 2), 2);
        assert_eq!(gcd_test(132, 2), 2);
        assert_eq!(gcd_test(2, 132), 2);
        assert_eq!(gcd_test(9, 132), 3);
        assert_eq!(gcd_test(19823785, 289374), 17);
    }
}
