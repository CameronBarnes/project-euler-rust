use primes::nth_prime;

#[tracing::instrument]
pub fn solve() -> String {
    nth_prime(10_001).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!("104743", solve());
    }
}
