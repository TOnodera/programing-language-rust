use walkdir::WalkDir;

async fn get_file_name(root: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut res: Vec<String> = vec![];
    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        let metsdata = entry.metadata().unwrap();
        if metsdata.is_dir() {
            res.push(format!("is dir: {:?}", entry.file_name()));
        } else {
            res.push(format!("is file: {:?}", entry.file_name()));
        }
    }
    Ok(res)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = get_file_name(".").await?;
    println!("{:?}", res);
    Ok(())
}
