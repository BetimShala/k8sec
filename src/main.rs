mod cli;
mod pull_command;

use cli::{Args, Action::*};
use structopt::StructOpt;

fn main() {
    // Get the command-line arguments.
    let Args {
        action,
        secret,
        context,
        namespace,
        output,
    } = Args::from_args();

    // Perform the action.
    match action {
        Pull { secret } => pull_command::run(secret, namespace.unwrap(), context.unwrap()),
        Push { file } => println!("push command"),
    }
}
