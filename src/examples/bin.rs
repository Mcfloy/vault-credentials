use dotenv::dotenv;

fn main() {
    dotenv().ok();
    vault_credentials::initialize();

    println!("{}", std::env::var("app.mongo.uri").unwrap());
}