use std::{
    fs::{self, DirEntry},
    time::Duration,
};
use tokio::time::sleep;

async fn read_dir(path: &str) -> Result<Vec<DirEntry>, Box<dyn std::error::Error>> {
    let dir = fs::read_dir(path)?;
    let mut files = vec![];
    for item in dir.into_iter() {
        files.push(item?);
    }
    Ok(files)
}

#[tokio::main]
async fn main() {
    let mut handlers = vec![];
    let handle = tokio::spawn(async {
        let res = read_dir(".").await;
        match res {
            Ok(r) => r,
            Err(_) => panic!("Error"),
        }
    });
    handlers.push(handle);

    for handler in handlers.into_iter() {
        let res = handler.await.unwrap();
        for r in res {
            println!("{:?}", r.metadata().unwrap());
        }
    }
}
