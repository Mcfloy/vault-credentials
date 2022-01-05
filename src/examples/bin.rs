use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    vault_credentials::initialize().await;

    println!("{}", std::env::var("mongo.url").unwrap());
}
