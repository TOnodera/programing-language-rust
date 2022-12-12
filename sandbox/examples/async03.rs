use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        sleep(Duration::from_secs(1)).await;
        return "done";
    });

    println!("hello,world");

    let result = handle.await.unwrap();
    println!("{}", result);
}
