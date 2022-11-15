//use rpassword::prompt_password;
use colored::*;
use serde::Deserialize;
use std::fs::{self, File};
use std::io;
use std::io::Read;
use std::io::Write;
use std::str::FromStr;
use toml::de::Error;

#[derive(Deserialize)]
struct Config {
    private_key: String,
    public_key: String,
}

pub fn add_repository(client: &String) -> Result<(), Error> {
    let repositories_config_file_path = format!(
        "{}/.grabber/grabber-repositories.toml",
        dirs::home_dir().unwrap().display()
    );

    let mut platform: String = String::new();
    print!("{}", "Specify a platform: ".bold());
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut platform)
        .expect("Error reading from STDIN");
    platform.pop();

    let mut file =
        File::open(&repositories_config_file_path).expect("ERROR: Please run grabber setup first");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("msg");

    println!(
        "{}",
        "Use 'quit' to stop adding repositories"
            .truecolor(255, 171, 0)
            .bold()
    );

    //let toml = toml::from_str(&mut contents)?;
    let mut continue_adding_repositories: i32 = 0;
    while continue_adding_repositories == 0 {
        match toml_edit::Document::from_str(&mut contents) {
            Ok(mut file) => {
                match file[&client][&platform]["repositories"].as_array_mut() {
                    None => eprintln!("ERROR: Unable to convert to TOML data"),
                    Some(repositories) => {
                        let mut repository_url: String = String::new();
                        print!("{}", "Enter repository SSH url: ".bold());
                        let _ = io::stdout().flush();
                        io::stdin()
                            .read_line(&mut repository_url)
                            .expect("Error reading from STDIN");
                        repository_url.pop();

                        //repositories.push(repository_url);
                        match &repository_url.eq("quit") {
                            true => continue_adding_repositories += 1,
                            false => {
                                repositories.push(repository_url);
                                fs::write(&repositories_config_file_path, file.to_string())
                                    .expect("ERROR: Unable to write to ~/.grabber/grabber-repositories.toml");
                            }
                        }
                    }
                }
            }
            Err(_) => println!("ERROR: Unable to edit document"),
        }
    }
    Ok(())
}

pub fn add_platform_repository(client: &String, platform: &Option<String>) -> Result<(), Error> {
    let platform_exists = {
        if platform.is_none() {
            true
        } else {
            false
        }
    };
    Ok(())

    

/* 
    let repositories_config_file_path = format!(
        "{}/.grabber/grabber-repositories.toml",
        dirs::home_dir().unwrap().display()
    );
    let mut repository_url: String = String::new();
    print!("{}", "Enter repository SSH url: ".bold());
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut repository_url)
        .expect("Error reading from STDIN");

    let mut file = File::open(&repositories_config_file_path).expect("msg");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("msg");

    let mut continue_adding_repositories: i32 = 0;
    while continue_adding_repositories == 0 {
        match toml_edit::Document::from_str(&mut contents) {
            Ok(mut file) => {
                match file[&client][&platform]["repositories"].as_array_mut() {
                    None => eprintln!("ERROR: Unable to convert to TOML data"),
                    Some(repositories) => {
                        let mut repository_url: String = String::new();
                        print!("{}", "Enter repository SSH url: ".bold());
                        let _ = io::stdout().flush();
                        io::stdin()
                            .read_line(&mut repository_url)
                            .expect("Error reading from STDIN");
                        repository_url.pop();

                        //repositories.push(repository_url);
                        match &repository_url.eq("quit") {
                            true => continue_adding_repositories += 1,
                            false => {
                                repositories.push(repository_url);
                                fs::write(&repositories_config_file_path, file.to_string())
                                    .expect("ERROR: Unable to write to ~/.grabber/grabber-repositories.toml");
                            }
                        }
                    }
                }
            }
            Err(_) => println!("ERROR: Unable to edit document"),
        }
    }
    Ok(())*/
}

/*

fn main() {
    let host = "bitbucket.org-nubersia";
    let config_file: String  = fs::read_to_string(format!("/Users/hruiz/.grabber/grabber-repositories.toml", &host)).unwrap();
    let config: Config = toml::from_str(&config_file)
        .expect("ERROR: Parsing file");

    let repo_url = "git@bitbucket.org:nubersiateam/terraform-bebetting.git";
    let repo_clone_path = "/Users/hruiz/`Grabber/Clientes/terraform-bebetting";

    let ssh_password: String = prompt_password("Enter passphrase for GitHub Key: ".bold().truecolor(255, 171, 0)).unwrap().into();

    let mut builder = RepoBuilder::new();
    let mut callbacks = RemoteCallbacks::new();
    let mut fetch_options = FetchOptions::new();
    callbacks.credentials(|_, _, _| {
        let credentials = Cred::ssh_key(
            "git",
            Some(Path::new(&config.public_key)),
            Path::new(&config.private_key),
            Some(&ssh_password)
        )
        .expect("Could not create credentials object");

        Ok(credentials)
    });

    fetch_options.remote_callbacks(callbacks);

    builder.fetch_options(fetch_options);

    println!("Cloning {} into {}", repo_url, repo_clone_path);


   match builder.clone(repo_url, Path::new(repo_clone_path)) {
    Ok(_) => println!("Repository has been cloned {}"),
    Err(_) => eprintln!("ERROR: Password error")
   }


    // Do things with `repo` here
}
*/
