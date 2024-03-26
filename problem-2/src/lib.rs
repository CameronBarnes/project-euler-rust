
#[tracing::instrument]
pub fn solve() -> String {
    let mut first: usize = 1;
    let mut second: usize = 1;
    let mut result: usize = 0;

    while second < 4_000_000 {
        let temp = second;
        second += first;
        first = temp;
        if second % 2 == 0 {
            result += second;
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!("4613732", solve());
    }
}
