mod cli;
mod pull_command;

use cli::{Action::*, Args};
use structopt::StructOpt;

fn main() {
    // Get the command-line arguments.
    let Args {
        action,
        secret: _,
        context,
        namespace,
        output,
    } = Args::from_args();

    let namespace = namespace.unwrap_or("default".into());
    let output = output.unwrap_or("".into());
    // Perform the action.
    match action {
        Pull { secret } => pull_command::run(secret, namespace, context.unwrap(), output),
        Push { file: _ } => println!("push command"),
    }
}
