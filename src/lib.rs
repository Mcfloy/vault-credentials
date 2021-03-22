mod token_retriever;
mod secret_retriever;
mod credentials;

use crate::credentials::Credentials;
use crate::token_retriever::TokenRetriever;
use crate::secret_retriever::SecretRetriever;

/// # Vault Credentials
/// Rust Library that fetch secrets from Vault and load them as environment variables.
/// Inspired by [Spring Cloud Vault](https://cloud.spring.io/spring-cloud-vault/reference/html/#vault.config.authentication).
pub fn initialize() {
    let vault_credentials = Credentials::new();
    let auth_token = TokenRetriever::retrieve_token(vault_credentials.clone());
    SecretRetriever::env_setter(vault_credentials, auth_token);
}
