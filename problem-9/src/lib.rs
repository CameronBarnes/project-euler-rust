
#[tracing::instrument]
pub fn solve() -> String {
    for c in 413_i32..=500 {
        let c2 = c.pow(2);
        let c1000 = 1000 - c;
        for a in 1.max(c1000 - ( c - 1))..=(332.min(c1000 / 2) + 1) {
            let b = c1000 - a;
            if a.pow(2) + b.pow(2) == c2 {
                return (a * b * c).to_string();
            }
        }
    }
    String::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!("31875000", solve());
    }
}
