
#[tokio::main]
async fn main() {
    req_test().await.unwrap();
}

async fn req_test() -> Result<(), reqwest::Error> {
    let body = reqwest::get("https://www.rust-lang.org").await?;
    let t = body.text().await?;
    
    println!("{}", t);
    Ok(())
}

