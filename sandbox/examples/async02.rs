use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        sleep(Duration::from_secs(1)).await;
        println!("1 sec");
    });

    println!("hello,world");

    sleep(Duration::from_secs(2)).await;
    println!("2 sec");
}
