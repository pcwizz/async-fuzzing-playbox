#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn simple_target_test() {
        assert_eq!(simple_target(0, b"Hello").await,b'H');
        assert_ne!(simple_target(0, b"Hello").await,b'h');
        assert_eq!(simple_target(3, b"Hello").await,b'l')
    }
}


pub async fn simple_target(i: usize, s: &[u8]) -> u8 {
    s[i]
}