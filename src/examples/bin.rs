fn main() {
    std::env::set_var("VAULT_ADDR", "http://127.0.0.1:8200");
    // std::env::set_var("VAULT_TOKEN", "s.A4b7Q7cGtNvC1ZZVGyNDANIa");
    std::env::set_var("VAULT_PATH", "hello");
    std::env::set_var("VAULT_TYPE", "app_role");
    std::env::set_var("VAULT_ROLE_ID", "9bf0581f-d48f-5f13-8608-533ba207ec80");
    std::env::set_var("VAULT_SECRET_ID", "08844018-213d-408d-588c-c85ecc216247");

    vault_credentials::initialize();

    println!("{}", std::env::var("github.com.api-key").unwrap());
}