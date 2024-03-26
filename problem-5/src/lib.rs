fn check(num: usize) -> bool {
    for x in 1..=20 {
        if num % x != 0 {
            return false;
        }
    }
    true
}

#[tracing::instrument]
pub fn solve() -> String {
    let mut num = 20;
    loop {
        if check(num) {
            return num.to_string();
        }
        num += 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!("232792560", solve());
    }
}
