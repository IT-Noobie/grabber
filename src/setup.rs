use colored::*;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use toml::map::Map;
use toml::Value;

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
    let mut platform_ssh_key_alias: String = String::new();
    print!("{}", "Enter platform ssh key alias: ".bold());
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut platform_ssh_key_alias)
        .expect("Error reading from STDIN");

    let mut private_key: String = String::new();
    print!("{}", "Enter private key absolute path: ".bold());
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut private_key)
        .expect("Error reading from STDIN");

    let mut public_key: String = String::new();
    print!("{}", "Enter public key absolute path: ".bold());
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut public_key)
        .expect("Error reading from STDIN");

    // remove carriage return of stdin
    platform_ssh_key_alias.pop();
    private_key.pop();
    public_key.pop();

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
    fs::create_dir(&home_directory_path).expect("ERROR: Unable to create grabber directory");
    println!("Directory has been created at ~/.grabber/");

    let platform_config_file_path: String = format!("{}/grabber-config.toml", &home_directory_path);
    let repository_file_path: String =
        format!("{}/grabber-repositories.toml", &home_directory_path);
    fs::File::create(&platform_config_file_path)
        .expect("ERROR: Unable to create grabber config file");
    fs::File::create(&repository_file_path).expect("ERROR: Unable to create repositories file");

    println!(
        "grabber config file has been successfully created at: {}",
        platform_config_file_path
    );
    println!(
        "Repositories file has been successfully created at: {}\n",
        repository_file_path
    );
}
