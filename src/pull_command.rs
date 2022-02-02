use std::collections::HashMap;
use std::fs::OpenOptions;
use std::path::PathBuf;

#[path = "kubectl.rs"]
mod kubectl;

use self::kubectl::Secret;

pub fn run(secret: String, namespace: String, context: String, file: PathBuf) {
    let secrets = kubectl::get_secrets(&secret, &context, &namespace);
    println!("{}", file.as_os_str().len());
    if secrets.len() <= 0 {
        return;
    }

    if file.as_os_str().len() <= 0 {
        write_to_console(secrets);
    } else {
        write_to_file(&secrets, &secret, &namespace, &context, file);
    }
}

fn write_to_console(secrets: Vec<Secret>) {
    for secret in secrets {
        println!("{:#<1$}", "", 30);
        println!("Name: {} \nValue: {}", secret.name, secret.value);
        println!("{:#<1$}", "", 30);
    }
}

fn write_to_file(
    secrets: &Vec<Secret>,
    secret: &String,
    namespace: &String,
    context: &String,
    file_path: PathBuf,
) {
    // Open the file.
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path)
        .unwrap();

    let mut yaml_secrets = HashMap::new();
    yaml_secrets.insert("namespace", namespace);
    yaml_secrets.insert("context", context);
    yaml_secrets.insert("secret", secret);

    for s in secrets {
        yaml_secrets.insert(&s.name, &s.value);
    }

    serde_yaml::to_writer(file, &yaml_secrets);
}
