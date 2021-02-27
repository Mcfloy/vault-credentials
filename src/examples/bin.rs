use dotenv::dotenv;

fn main() {
    dotenv().ok();
    vault_credentials::initialize();

    println!("{}", std::env::var("github.com.api-key").unwrap());
}