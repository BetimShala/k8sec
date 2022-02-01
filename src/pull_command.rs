use std::fs::OpenOptions;
use std::io::Result;
use std::path::PathBuf;

#[path = "kubectl.rs"]
mod kubectl;

use self::kubectl::Secret;

pub fn run(secret: String, namespace: String, context: String, file: PathBuf) {
    let secrets = kubectl::get_secrets(secret, context, namespace);
    if secrets.len() <= 0 {
        return;
    }

    if file.as_os_str().len() <= 0 {
        print_raw(secrets);
    } else {
        write_to_file(secrets, file);
    }
}

fn print_raw(secrets: Vec<Secret>) {
    for secret in secrets {
        println!("Name: {} \nValue: {}", secret.name, secret.value)
    }
}

fn write_to_file(secrets: Vec<Secret>, file_path: PathBuf) -> Result<()> {
    // Open the file.
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path)?;

    serde_json::to_writer(file, &secrets)?;
    Ok(())
}
