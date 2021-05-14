use std::env;
use std::fs;
use serde_json::json;
use crate::Credentials;
use std::collections::HashMap;

pub struct TokenRetriever {}

impl TokenRetriever {
    pub async fn retrieve_token(vault_credentials: Credentials) -> String {
        let authentication_type = match env::var("VAULT_TYPE") {
            Ok(value) => value,
            _ => String::from("token")
        };
        let request_url = {
            format!("{}/v1/auth/{}/login", String::from(vault_credentials.vault_addr), authentication_type)
        };

        let mut headers = HashMap::new();
        if let Some(namespace) = vault_credentials.vault_namespace {
            headers.insert(String::from("x-vault-namespace"), namespace);
        }

        if authentication_type == "token" {
            String::from(env::var("VAULT_TOKEN")
                .expect("Cannot get environment variable VAULT_TOKEN").as_str())
        } else {
            let payload = TokenRetriever::generate_payload(authentication_type.as_str());
            TokenRetriever::call_vault_login(&request_url, Some(headers), &payload).await
        }
    }

    fn generate_payload(authentication_type: &str) -> serde_json::Value {
        match authentication_type {
            "approle" => {
                let role_id = env::var("VAULT_ROLE_ID")
                    .expect("Cannot get environment variable VAULT_ROLE_ID");
                let secret_id = env::var("VAULT_SECRET_ID")
                    .expect("Cannot get environment variable VAULT_SECRET_ID");

                json!({
                    "role_id": role_id,
                    "secret_id": secret_id
                })
            },
            "userpass" | "ldap" => {
                let username = env::var("VAULT_USERNAME")
                    .expect("Cannot get environment variable VAULT_USERNAME");
                let password = env::var("VAULT_PASSWORD")
                    .expect("Cannot get environment variable VAULT_PASSWORD");

                json!({
                    "username": username,
                    "password": password
                })
            },
            "kubernetes" => {
                let auth_path = env::var("VAULT_K8S_AUTH_PATH")
                    .expect("Cannot get environment variable VAULT_K8S_AUTH_PATH");
                let role_name = env::var("VAULT_ROLE_NAME")
                    .expect("Cannot get environment variable VAULT_ROLE_NAME");

                let jwt = fs::read_to_string(auth_path)
                    .expect("Cannot kubernetes auth file from path");

                json!({
                    "jwt": jwt,
                    "role": role_name
                })
            },
            _ => panic!("{} is not supported.", authentication_type)
        }
    }

    async fn call_vault_login(request_url: &str, headers_option: Option<HashMap<String, String>>, payload: &serde_json::Value) -> String {
        let mut request_builder = reqwest::Client::new()
            .post(request_url);

        if let Some(headers) = headers_option {
            for (name, value) in &headers {
                request_builder = {
                    request_builder.header(name, value)
                };
            }
        }

        let response: serde_json::Value = request_builder
            .json(payload)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        if let Some(errors) = response.get("errors") {
            panic!("Cannot retrieve token: {}", errors.as_array().unwrap().first().unwrap());
        }

        let client_token = response
            .get("auth").unwrap()
            .get("client_token").unwrap();

        String::from(client_token.as_str().unwrap())
    }
}
