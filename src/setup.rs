use colored::*;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use toml::map::Map;
use toml::Value;

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

pub fn setup() {
    create_grabber_directory();
    let mut n: i32 = 0;
    while n == 0 {
        match create_config_file() {
            Ok(_) => println!(),
            Err(_) => println!("ERROR: Unable to create config file at ~/.grabber"),
        }
        let mut continue_creating: String = String::new();
        print!(
            "{}",
            "Do you want to add another SSH configuration [y/N]? "
                .bold()
                .truecolor(255, 171, 0)
        );
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut continue_creating)
            .expect("Error reading from STDIN");
        // remove carriage returrn
        continue_creating.pop();
        if !continue_creating.eq("y") {
            n += 1;
        }
    }
}

fn create_config_file() -> std::io::Result<()> {
    let message: String = String::from("Enter platform ssh key alias: ");
    let platform_ssh_key_alias: String = Input { message }.input();

    let message: String = String::from("Enter private key absolute path: ");
    let private_key: String = Input { message }.input();

    let message: String = String::from("Enter public key absolute path: ");
    let public_key: String = Input { message }.input();

    let mut config: Map<String, Value> = Map::new();
    let mut values: Map<String, Value> = Map::new();

    values.insert(String::from("private_key"), Value::String(private_key));
    values.insert(String::from("public_key"), Value::String(public_key));
    config.insert(platform_ssh_key_alias, Value::Table(values));

    let config_file = toml::to_string(&config).expect("ERROR: Unable to parse data to TOML");

    let platform_config_file_path: String = format!(
        "{}/.grabber/grabber-config.toml",
        dirs::home_dir().unwrap().display()
    );

    let mut ssh_config_file = OpenOptions::new()
        .append(true)
        .open(platform_config_file_path)
        .expect("ERROR: Unable to open ssh config file. ¿Does it exist?");

    ssh_config_file
        .write_all(config_file.as_bytes())
        .expect("ERROR: Unable to create config file");
    Ok(())
}

fn create_grabber_directory() {
    let home_directory_path = format!("{}/.grabber", dirs::home_dir().unwrap().display());
    match fs::create_dir(&home_directory_path) {
        Ok(_) => println!("Directory has been created at ~/.grabber/"),
        Err(_) => eprintln!("Directory already exists"),
    }

    let platform_config_file_path: String = format!("{}/grabber-config.toml", &home_directory_path);
    let repository_file_path: String =
        format!("{}/grabber-repositories.toml", &home_directory_path);

    match fs::File::create(&platform_config_file_path) {
        Ok(_) => println!(
            "Grabber config file has been successfully created at: {}",
            platform_config_file_path
        ),
        Err(_) => eprintln!("ERROR: File already exists"),
    }

    match fs::File::create(&repository_file_path) {
        Ok(_) => println!(
            "Repositories file has been successfully created at: {}",
            repository_file_path
        ),
        Err(_) => eprintln!("ERROR: File already exists"),
    }
}
