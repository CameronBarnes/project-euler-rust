
#[tracing::instrument]
pub fn solve() -> String {
    let result: usize = (3..1000).filter(|x| x % 3 == 0 || x % 5 == 0).sum();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!("233168", solve());
    }
}
