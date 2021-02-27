# Vault Credentials
Rust Library that fetch secrets from Vault and load them as environment variables.
Inspired by [Spring Cloud Vault](https://cloud.spring.io/spring-cloud-vault/reference/html/#vault.config.authentication).

## Getting started
We will assume that you want to retrieve some secrets from your local [Vault Server](https://learn.hashicorp.com/tutorials/vault/getting-started-dev-server?in=vault/getting-started).

This is the json secret located in `secret/hello` (from Vault perspective, either by using the Vault UI or Vault CLI)
```json
{
  "my-key": "my-value",
  "github.com": {
    "api-key": "123456",
    "base-url": "http://localhost:8080"
  }
}
```

In your program you must provide the environment variables required to make a connection to the Vault Server and retrieve the token.
You can use the [.dotenv](https://crates.io/crates/dotenv) crate and put the variables in a *.env* file.
```
VAULT_ADDR=http://127.0.0.1:8200
VAULT_PATH=hello
VAULT_TYPE=approle
VAULT_ROLE_ID=9bf0581f-[...]-533ba207ec80
VAULT_SECRET_ID=55473ff2-[...]-0ab9ae6e499b
```

To use the vault_credentials crate in your program, import it and call the `initialize` method.
```rust
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    vault_credentials::initialize();

    println!("{}", std::env::var("github.com.api-key").unwrap());
    // Output: 123456
}
```

## Authentication types
You can use other types of authentication by using `VAULT_TYPE`. (default is set to `token`)

|Vault Type|Required environment variables|
|---|---|
|`token`|`VAULT_TOKEN`|
|`approle`|`VAULT_ROLE_ID`,`VAULT_SECRET_ID`|
|`kubernetes`|`VAULT_K8S_AUTH_PATH`,`VAULT_ROLE_NAME`|
|`userpass`,`ldap` |`VAULT_USERNAME`, `VAULT_PASSWORD`|