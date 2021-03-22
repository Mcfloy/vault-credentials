use std::env;

#[derive(Clone)]
pub struct Credentials {
    pub(crate) vault_addr: String,
    pub(crate) vault_namespace: Option<String>
}

impl Credentials {
    pub fn new() -> Self {
        let vault_addr = env::var("VAULT_ADDR")
            .expect("Cannot get environment variable VAULT_ADDR");

        let mut entity = Credentials {
            vault_addr,
            vault_namespace: None
        };

        let vault_namespace_env = env::var("VAULT_NAMESPACE");

        if let Ok(namespace) = vault_namespace_env {
            entity.vault_namespace = Some(namespace);
        }

        entity
    }
}