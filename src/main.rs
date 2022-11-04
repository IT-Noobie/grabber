use std::process::exit;

use clap::{Parser, Subcommand};

mod list;
mod new;
mod setup;

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
        client: Option<String>,
        #[clap(short, long)]
        /// Name of the platform key alias
        platform: Option<String>,
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
        Commands::List { client, platform } => {
            if client.is_some() && platform.is_some() {
                match list::client_platform_repositories(
                    &client.to_owned().unwrap(),
                    &platform.to_owned().unwrap(),
                ) {
                    Ok(_) => exit(0),
                    Err(err) => {
                        eprintln!("ERROR: {}", err);
                        exit(1);
                    }
                };
            };
            if client.is_some() && platform.is_none() {
                match list::client_platform(&client.to_owned().unwrap()) {
                    Ok(_) => exit(0),
                    Err(err) => {
                        eprintln!("ERROR: {}", err);
                        exit(1);
                    }
                }
            }
            if client.is_none() && platform.is_some() {
                match list::platform_key_alias_config(&platform.to_owned().unwrap()) {
                    Ok(_) => exit(0),
                    Err(err) => {
                        eprintln!("ERROR: {}", err);
                        exit(1);
                    }
                };
            };
            match list::platforms() {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("ERROR: {}", err);
                    exit(1);
                }
            };
        }
        Commands::New { client } => {
            println!("New client {:?} will be configured", client);
            new::new(client)
        }
        Commands::Setup => setup::setup(),
    }
}
