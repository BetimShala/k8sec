use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Pulls secrets from kubernetes to a local file
    Pull {
        /// The name of the secret in kubernetes
        #[structopt()]
        secret: String,
    },
    /// Pushes secrets from a local file into kubernetes
    Push {
        /// The file to read from
        #[structopt(parse(from_os_str), short, long)]
        file: Option<std::path::PathBuf>,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "k8sec",
    about = "A command line app to manage k8s secrets easy way written in Rust"
)]
pub struct Args {
    #[structopt(subcommand)]
    pub action: Action,

    /// The kubectl config context to use
    #[structopt(short, long)]
    pub context: Option<String>,

    /// The name of the secret in kubernetes
    #[structopt(short, long)]
    pub secret: Option<String>,

    /// The namespace of services in kubernetes
    #[structopt(short, long)]
    pub namespace: Option<String>,

    /// The file to write to
    #[structopt(parse(from_os_str), short, long)]
    pub output: Option<std::path::PathBuf>,
}
