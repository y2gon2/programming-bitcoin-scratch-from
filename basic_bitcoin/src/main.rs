
// #[tokio::main]
// async fn main() {
//     req_test().await.unwrap();
// }

// async fn req_test() -> Result<(), reqwest::Error> {
//     let body = reqwest::get("https://www.rust-lang.org").await?;
//     let t = body.text().await?;
    
//     println!("{}", t);
//     Ok(())
// }

use sha2::{Sha256, Digest};

fn main() {
    // create a Sha256 object
    let mut hasher = Sha256::new();
    let a: Vec<u8> = vec![1, 2, 3];
    hasher.update(a);

    // read hash digest and consume hasher
    let result = hasher.finalize();

    println!("{:?}", result);
}