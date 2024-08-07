#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn simple_target_test() {
        assert_eq!(simple_target(0, b"Hello").await, 4)
    }
}

pub async fn simple_target(i: usize, s: &[u8]) -> u8 {
    if i % 42 == 23 && s.len() > 500 {
        for c in s {
            if *c as usize * i == 3000 {
                panic!()
            }
        }
    }
    4
}
