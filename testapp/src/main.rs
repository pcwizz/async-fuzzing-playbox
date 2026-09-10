use std::time::Duration;

use tokio::time::sleep;

#[tokio::main]
async fn main() {
    foo(5).await
}

async fn foo(i: u32) {
    println!("{:?}", i);
    sleep(Duration::from_millis(12)).await;
    println!("{:?}", i * i);
}
