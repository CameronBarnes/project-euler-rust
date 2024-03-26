fn largest_prime_factor(mut num: usize) -> usize {
    let mut largest = 0;
    let mut d = 2;
    while num > 1 {
        while num % d == 0 {
            largest = largest.max(d);
            num /= d;
        }
        if d > 2 {
            d += 2;
        } else {
            d += 1;
        }
        if d * d > num && num > 1 {
            largest = largest.max(num);
            break;
        }
    }
    largest
}

#[tracing::instrument]
pub fn solve() -> String {
    largest_prime_factor(600_851_475_143).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!("6857", solve());
    }
}
