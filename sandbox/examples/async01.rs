use std::time::Duration;
use tokio::time::sleep;

async fn hello() -> String {
    sleep(Duration::from_millis(1000)).await;
    "hello, async fn".to_string()
}

#[tokio::main]
async fn main() {
    let greeting: String = hello().await;
    println!("{}", greeting);

    let world = async {
        println!("hello, async block");
    };

    world.await;
}
