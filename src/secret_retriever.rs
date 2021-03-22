use crate::Credentials;
use std::env;

pub struct SecretRetriever {}

impl SecretRetriever {
    pub fn env_setter(vault_credentials: Credentials, auth_token: String) {
        let vault_path = std::env::var("VAULT_PATH").unwrap();

        let request_uri = {
            format!("{}/v1/secret/data/{}", String::from(vault_credentials.vault_addr), vault_path)
        };

        let mut request_builder = reqwest::blocking::Client::new()
            .get(&request_uri)
            .header("X-Vault-Token", auth_token);

        if let Some(namespace) = vault_credentials.vault_namespace {
            request_builder = {
                request_builder.header("X-Vault-Namespace", namespace)
            };
        }

        let response: serde_json::Value = request_builder.send()
            .unwrap()
            .json()
            .unwrap();

        if let Some(errors) = response.get("errors") {
            panic!("Cannot retrieve token: {}", errors.as_array().unwrap().first().unwrap());
        }

        let object = response.get("data").unwrap().get("data").unwrap().as_object().unwrap();

        for (key, value) in object {
            SecretRetriever::add_to_env(key, value);
        }
    }

    fn add_to_env(key_path: &String, value: &serde_json::Value) {
        if value.is_object() {
            for (key, value) in value.as_object().unwrap() {
                SecretRetriever::add_to_env(&format!("{}.{}", key_path, key), value);
            }
        } else if !value.is_array() {
            env::set_var(key_path, value.as_str().unwrap());
        } else {
            unimplemented!("Vault secrets shouldn't have arrays and must use objects to ensure unique keys.");
        }
    }
}
