//use rpassword::prompt_password;
use colored::*;
use serde::Deserialize;
use std::fs::{self, File};
use std::io;
use std::io::Read;
use std::io::Write;
use std::process::exit;
use std::str::FromStr;
use toml::de::Error;

#[derive(Deserialize)]
//struct Config {
//    private_key: String,
//    public_key: String,
//}
struct Input {
    message: String,
}

impl Input {
    fn input(&self) -> String {
        let mut value: String = String::new();
        print!("{}", self.message.bold());
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut value)
            .expect("Error reading from STDIN");
        value.pop();
        value
    }
}

pub fn add_platform_repository(client: &String, platform: &Option<String>) -> Result<(), Error> {
    match platform {
        Some(platform_name) => {
            match add(client, platform_name) {
                Ok(_) => println!("Successfully Added repositories"),
                Err(_) => eprintln!("ERROR: Unable to add repositories"),
            }
            Ok(())
        }
        None => {
            let message: String = String::from("Specify a platform: ");
            let platform_name: String = Input { message }.input();
            match add(client, &platform_name) {
                Ok(_) => println!("Successfully Added repositories"),
                Err(_) => eprintln!("ERROR: Unable to add repositories"),
            }
            Ok(())
        }
    }
}

fn add(client: &String, platform: &String) -> Result<(), Error> {
    let repositories_config_file_path = format!(
        "{}/.grabber/grabber-repositories.toml",
        dirs::home_dir().unwrap().display()
    );

    let mut file =
        File::open(&repositories_config_file_path).expect("ERROR: Please run grabber setup first");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("msg");

    let mut continue_adding_repositories: i32 = 0;
    while continue_adding_repositories == 0 {
        match toml_edit::Document::from_str(&contents) {
            Ok(mut file) => match file[client][platform]["repositories"].as_array_mut() {
                None => {
                    eprintln!("ERROR: client or platform doesn't exist. Run grabber list -c <CLIENT> to list platforms");
                    exit(3)
                }
                Some(repositories) => {
                    println!(
                        "{}",
                        "Use 'quit' to stop adding repositories"
                            .truecolor(255, 171, 0)
                            .bold()
                    );
                    let message: String = String::from("Enter repository SSH url: ");
                    let repository_url: String = Input { message }.input();

                    match &repository_url.eq("quit") {
                        true => continue_adding_repositories += 1,
                        false => {
                            repositories.push(repository_url);
                            fs::write(&repositories_config_file_path, file.to_string()).expect(
                                "ERROR: Unable to write to ~/.grabber/grabber-repositories.toml",
                            );
                        }
                    }
                }
            },
            Err(_) => println!("ERROR: Unable to edit document"),
        }
    }
    Ok(())
}
