
#[tracing::instrument]
pub fn solve() -> String {
    let mut square = (1..=100).sum::<usize>().pow(2);
    square -= (1..=100).map(|x: usize| x.pow(2)).sum::<usize>();
    square.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!("25164150", solve());
    }
}
