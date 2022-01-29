#[path = "kubectl.rs"] mod kubectl;

pub fn run(secret: String, namespace: String, context: String) {
    kubectl::get_secrets(secret, context, namespace);
}
