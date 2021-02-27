use std::env;
use serde_json::json;
use reqwest;

pub fn initialize() {
   retrieve_token();

   env_setter();
}

pub fn retrieve_token() {
    let vault_addr = env::var("VAULT_ADDR")
        .expect("Cannot get environment variable VAULT_ADDR");

    let authentication_type = match env::var("VAULT_TYPE") {
        Ok(value) => value,
        _ => String::from("token")
    };
    let request_url = format!("{}/v1/auth/{}/login", vault_addr, authentication_type);

    match authentication_type.as_str() {
        "token" => {
            env::var("VAULT_TOKEN")
                .expect("Cannot get environment variable VAULT_TOKEN");
        },
        "approle" => {
            let role_id = env::var("VAULT_ROLE_ID")
                .expect("Cannot get environment variable VAULT_ROLE_ID");
            let secret_id = env::var("VAULT_SECRET_ID")
                .expect("Cannot get environment variable VAULT_SECRET_ID");

            let payload = json!({
                "role_id": role_id,
                "secret_id": secret_id
            });

            call_vault_login(&request_url, &payload);
        },
        "userpass" | "ldap" => {
            let username = env::var("VAULT_USERNAME")
                .expect("Cannot get environment variable VAULT_USERNAME");
            let password = env::var("VAULT_PASSWORD")
                .expect("Cannot get environment variable VAULT_PASSWORD");

            let payload = json!({
                "username": username,
                "password": password
            });

            call_vault_login(&request_url, &payload);
        },
        "kubernetes" => {
            let role = env::var("VAULT_ROLE_NAME")
                .expect("Cannot get environment variable VAULT_ROLE_NAME");
            let path = env::var("VAULT_K8S_AUTH_PATH")
                .expect("Cannot get environment variable VAULT_K8S_AUTH_PATH");
            let jwt = std::fs::read_to_string(path);

            let payload = json!({
                "role": role,
                "jwt": jwt
            });
            call_vault_login(&request_url, &payload);
        }
        _ => panic!("{} is not supported.", authentication_type)
    }
}

fn call_vault_login(request_url: &str, payload: &serde_json::Value) {
    println!("Calling vault login {} with payload {:?}", request_url, payload);

    let response: serde_json::Value = reqwest::blocking::Client::new()
        .post(request_url)
        .json(payload)
        .send()
        .unwrap()
        .json()
        .unwrap();

    if let Some(errors) = response.get("errors") {
        panic!("Cannot retrieve token: {}", errors.as_array().unwrap().first().unwrap());
    }

    let client_token = response
        .get("auth").unwrap()
        .get("client_token").unwrap();
    env::set_var("VAULT_TOKEN", client_token.as_str().unwrap());
}

pub fn env_setter() {
    let vault_addr = std::env::var("VAULT_ADDR").unwrap();
    let vault_token = std::env::var("VAULT_TOKEN").unwrap();
    let vault_path = std::env::var("VAULT_PATH").unwrap();

    let request_uri = format!("{}/v1/secret/data/{}", vault_addr, vault_path);

    let response: serde_json::Value = reqwest::blocking::Client::new()
        .get(&request_uri)
        .header("X-Vault-Token", vault_token)
        .send()
        .unwrap()
        .json()
        .unwrap();

    if let Some(errors) = response.get("errors") {
        panic!("Cannot retrieve token: {}", errors.as_array().unwrap().first().unwrap());
    }

    let object = response.get("data").unwrap().get("data").unwrap().as_object().unwrap();

    for (key, value) in object {
        add_to_env(key, value);
    }
}

fn add_to_env(key_path: &String, value: &serde_json::Value) {
    if value.is_object() {
        for (key, value) in value.as_object().unwrap() {
            add_to_env(&format!("{}.{}", key_path, key), value);
        }
    } else if !value.is_array() {
        env::set_var(key_path, value.as_str().unwrap());
    } else {
        unimplemented!("Vault secrets shouldn't have arrays and must use objects to ensure unique keys.");
    }
}