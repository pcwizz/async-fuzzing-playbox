use core::time;
use std::{clone, sync::Arc};

use tokio::io::AsyncReadExt;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn simple_target_test() {
        assert_eq!(simple_target(0, b"Hello").await, 4)
    }

    #[tokio::test]
    #[should_panic]
    async fn reading_target_test_bug() {
        let reader = tokio_test::io::Builder::new().read(&[0, 6, 2, 1, 45, 65, 3, 54, 5, 9, 0, 6, 2, 1, 45, 65, 3]).build();
        reading_target(reader).await
    }

    #[tokio::test]
    async fn reading_target_test_no_bug() {
        let reader = tokio_test::io::Builder::new().read(&[0, 6, 2, 1, 45, 65, 3, 5, 5, 9, 0, 5, 2, 1, 45, 65, 3]).build();
        reading_target(reader).await
    }

    #[tokio::test]
    async fn spawn_target_test() {
        assert_eq!(spawn_target(4,&[4]).await, 42)
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

pub async fn reading_target<Reader>(r: Reader)
where
    Reader: tokio::io::AsyncRead + Unpin,
{
    let mut reader = tokio::io::BufReader::new(r);
    let mut buffer = [0; 10];
    loop {
        let n = reader.read(&mut buffer).await.unwrap();
        if n == 0 {
            break;
        }
        if n == 7 {
            for i in 0..7{
                assert_ne!(buffer[i..i+2],[6,2]);
            }
        }
    }
}

pub async fn spawn_target(i: usize, s: &[u8]) -> u8
{
    let s =s.to_owned();
    let _ = tokio::spawn(async move {
        let _ = simple_target(i, &s).await;
     }).await;
    42
}