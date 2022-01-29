use std::process::{Command, Stdio};
pub fn get_secrets(secret: String, context: String, namespace: String) {
    let result = execute_command(format!(
        "get secret {} -c {} -n {} -o json",
        secret, context, namespace
    ));
    println!("result: {}", String::from_utf8(result.stdout).unwrap());
}

fn execute_command(command: String) -> std::process::Output {
    println!("command is: {}", command);
    let output = Command::new("kubectl")
        .arg(command)
        .stdout(Stdio::piped())
        .output()
        .expect("kubectl command failed to start");
    return output;
}
