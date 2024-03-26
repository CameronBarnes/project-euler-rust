
#[tracing::instrument]
fn is_palendrome(str: &[u8]) -> bool {
    let max = str.len() - 1;
    let half = str.len() / 2;
    for x in 0..half {
        if str[x] != str[max - x] {
            return false;
        }
    }
    true
}

fn find_largest() -> String {
    let mut largest = 0;
    for x in 100..=999 {
        for y in 100..=999 {
            let num = x * y;
            if is_palendrome(num.to_string().as_bytes()) && num > largest {
                largest = num;
            }
        }
    }
    largest.to_string()
}

#[tracing::instrument]
pub fn solve() -> String {
    find_largest()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!("906609", solve());
    }
}
