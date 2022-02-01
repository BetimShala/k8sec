use base64::decode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::process::Command;
use std::str::from_utf8;
use structopt::StructOpt;

pub fn get_secrets(secret: String, context: String, namespace: String) -> Vec<Secret> {
    if !check_if_namespace_exists(&namespace, &context) {
        panic!(
            "Error from server (NotFound): namespace '{}' not found",
            namespace
        )
    }

    let output = execute_command(format!(
        "get secret {} --context {} -n {} -o json",
        secret, context, namespace
    ));

    let secrets_str = String::from_utf8_lossy(&output.stdout).into_owned();
    // Parse the string of data into serde_json::Value.
    let secrets_json: Value = serde_json::from_str(&secrets_str).unwrap();

    let mut secrets: Vec<Secret> = Vec::new();
    for (key, value) in secrets_json["data"].as_object().unwrap() {
        secrets.push(Secret {
            name: key.to_string(),
            value: from_utf8(&decode(value.as_str().unwrap()).unwrap())
                .unwrap()
                .to_string(),
        });
    }

    return secrets;
}

fn check_if_namespace_exists(namespace: &String, context: &String) -> bool {
    let output = execute_command(format!(
        "get namespaces --context {} -o custom-columns=:metadata.name",
        context
    ));
    let namespaces_str = String::from_utf8_lossy(&output.stdout).into_owned();
    let namespaces = namespaces_str.split('\n').collect::<Vec<&str>>();
    return namespaces.iter().any(|&n| n == namespace);
}

fn execute_command(command: String) -> std::process::Output {
    let mut cmd = Command::new("sh");
    let output = cmd
        .arg("-c")
        .arg(format!("kubectl {}", command))
        .output()
        .expect("kubectl command failed to start");
    return output;
}

#[derive(Debug, StructOpt, Deserialize, Serialize)]
#[structopt()]
pub struct Secret {
    #[structopt()]
    pub name: String,

    #[structopt()]
    pub value: String,
}
