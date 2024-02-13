use dotenvy;
use std::env;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let environment = env::var("APP_ENVIRONMENT").unwrap();
    println!("{}", environment);
    Ok(())
}
