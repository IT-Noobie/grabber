use clap::{Parser, Subcommand};

mod setup;
mod new;

#[derive(Parser)]
#[clap(about, version, author)]
struct Value {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add one or multiple repositories to a client
    Add {
        #[clap(short, long)]
        /// Repository SSH string to clone it
        repo: String,
        #[clap(short, long)]
        /// Client to add the repository
        client: String,
    },
	/// List client platforms and repositories
	List {
		#[clap(short, long)]
		/// Name of the client to list
		client: String,
	},
	/// Adds a new Client
    New {
        #[clap(short, long)]
        /// Name of the client to add
        client: String,
    },
    /// Configure script files and directory. You must run this first.
    Setup,
}

fn main() {
    let value = Value::parse();
    match &value.command {
        Commands::Add { repo, client } => {
            println!(
                "New repositoriy {:?} will be added to client {:?}",
                repo, client
            )
        }
        Commands::List { client } => {
            println!("Platforms of client: {}", client)
        }
        Commands::New { client } => {
            println!("New client {:?} will be configured", client);
            new::new(client)
        }
        Commands::Setup => {
            setup::setup()
        }
    }
}
