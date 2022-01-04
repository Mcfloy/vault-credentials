use std::env;
use std::fs;
use serde_json::json;
use crate::Credentials;
use std::collections::HashMap;

pub async fn retrieve_token(vault_credentials: Credentials) -> String {
    let authentication_type = match env::var("VAULT_TYPE") {
        Ok(value) => value.to_lowercase(),
        _ => String::from("kubernetes")
    };
    let request_url = generate_request_url(&vault_credentials.vault_addr, &authentication_type);

    let mut headers = HashMap::new();
    if let Some(namespace) = vault_credentials.vault_namespace {
        headers.insert(String::from("x-vault-namespace"), namespace);
    }

    match authentication_type.as_str() {
        "token" => {
            String::from(env::var("VAULT_TOKEN")
                .expect("Cannot get environment variable VAULT_TOKEN").as_str())
        }
        r#type => {
            let payload = generate_payload(r#type);
            call_vault_login(&request_url, Some(headers), &payload).await
        }
    }
}

fn generate_request_url(vault_addr: &str, authentication_type: &str) -> String {
    match authentication_type {
        "kubernetes" => {
            match env::var("VAULT_K8S_AUTH_PATH") {
                Ok(path) => format!("{}/v1/auth/{}/login", String::from(vault_addr), path),
                _ => format!("{}/v1/auth/kubernetes/login", String::from(vault_addr))
            }
        }
        "ldap" => {
            let ldap_username = env::var("VAULT_USERNAME").unwrap();
            format!("{}/v1/auth/ldap/login/{}", String::from(vault_addr), ldap_username)
        }
        _ => {
            format!("{}/v1/auth/{}/login", String::from(vault_addr), authentication_type)
        }
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
        }
        "userpass" | "ldap" => {
            let username = env::var("VAULT_USERNAME")
                .expect("Cannot get environment variable VAULT_USERNAME");
            let password = env::var("VAULT_PASSWORD")
                .expect("Cannot get environment variable VAULT_PASSWORD");

            json!({
                    "username": username,
                    "password": password
                })
        }
        "kubernetes" => {
            let service_account_token = env::var("K8S_SERVICE_ACCOUNT_TOKEN")
                .unwrap_or(String::from("/var/run/secrets/kubernetes.io/serviceaccount"));
            let role_name = env::var("VAULT_ROLE_NAME")
                .expect("Cannot get environment variable VAULT_ROLE_NAME");

            let jwt = fs::read_to_string(service_account_token)
                .expect("Cannot kubernetes auth file from path");

            json!({
                    "jwt": jwt,
                    "role": role_name
                })
        }
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
