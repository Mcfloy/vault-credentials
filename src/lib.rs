use crate::credentials::Credentials;

mod token_retriever;
mod secret_retriever;
mod credentials;

/// Initialize the retrieving of the secrets to Vault.
/// You must provide at least the three following environment variables:
/// * `VAULT_ADDR`
/// * `VAULT_PATH`
/// * `VAULT_TYPE`
/// ```rust
/// use dotenv::dotenv;
///
/// #[tokio::main]
/// async fn main() {
///     dotenv().ok();
///     vault_credentials::initialize().await;
///
///     println!("{}", std::env::var("github.com.api-key").unwrap());
///     // Output: 123456
/// }
/// ```
/// # Using token
/// ```env
/// VAULT_TYPE=token
/// VAULT_TOKEN=[...]
/// ```
/// # Using app role
/// ```env
/// VAULT_TYPE=approle
/// VAULT_ROLE_ID=[...]
/// VAULT_SECRET_ID=[...]
/// ```
/// # Using kubernetes
/// ```env
/// VAULT_TYPE=kubernetes
/// VAULT_K8S_AUTH_PATH?=kubernetes-id
/// K8S_SERVICE_TOKEN=/path/to/k8s.json
/// VAULT_ROLE_NAME=[...]
/// ```
/// # Using user pass
/// ```env
/// VAULT_TYPE=userpass
/// VAULT_USERNAME=[...]
/// VAULT_PASSWORD=[...]
/// ```
/// # Using ldap
/// ```env
/// VAULT_TYPE=ldap
/// VAULT_USERNAME=[...]
/// VAULT_PASSWORD=[...]
/// ```
/// # Optional namespace
/// You can specify a namespace using `VAULT_NAMESPACE` environment variable.
pub async fn initialize() {
    let vault_credentials = Credentials::new();
    let auth_token = token_retriever::retrieve_token(vault_credentials.clone()).await;
    secret_retriever::retrieve(vault_credentials, auth_token).await;
}
