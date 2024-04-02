use primes::block_sieve;

#[tracing::instrument]
pub fn solve() -> String {
    block_sieve(2_000_000).iter().sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!("142913828922", solve());
    }
}
