use colored::*;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use toml::map::Map;
use toml::Value;

struct Config {
    platform_ssh_key_alias: String,
    repositories: Vec<Value>,
}

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

impl Config {
    fn add_platform(self) -> Map<String, Value> {
        let mut platform: Map<String, Value> = Map::new();
        let mut repositories: Map<String, Value> = Map::new();
        repositories.insert(
            String::from("repositories"),
            Value::Array(self.repositories),
        );
        platform.insert(self.platform_ssh_key_alias, Value::Table(repositories));
        platform
    }
}

pub fn new(client_name: &String) {
    let repositories_config_file = format!(
        "{}/.grabber/grabber-repositories.toml",
        dirs::home_dir().unwrap().display()
    );
    let mut file = OpenOptions::new()
        .append(true)
        .open(repositories_config_file)
        .expect("ERROR: Run 'grabber setup' to configure the script");

    let mut add_platform: i32 = 0;

    while add_platform == 0 {
        let mut repositories: Vec<Value> = Vec::new();
        let mut continue_adding_repositories: i32 = 0;

        let message: String = String::from("Enter platform ssh key alias: ");
        let platform_ssh_key_alias: String = Input { message }.input();
    
        println!(
            "{}",
            "Use 'quit' to stop adding repositories"
                .truecolor(255, 171, 0)
                .bold()
        );

        while continue_adding_repositories == 0 {
            let message: String = String::from("Enter ssh repository url: ");
            let repository_url: String = Input { message }.input();

            match &repository_url.eq("quit") {
                true => continue_adding_repositories += 1,
                false => repositories.push(Value::String(repository_url)),
            }
        }
        let config: Config = Config {
            platform_ssh_key_alias,
            repositories,
        };
        let platform = config.add_platform();

        let mut client: Map<String, Value> = Map::new();
        client.insert(
            client_name.clone().to_ascii_lowercase(),
            Value::Table(platform),
        );

        let toml_content = toml::to_string(&client).expect("ERROR: Parse TOML error");

        file.write_all(toml_content.as_bytes())
            .expect("ERROR: Unable to write TOML file");

        let mut continue_adding_platform: String = String::new();
        print!(
            "{}",
            "Do you want to to add another Platform [y/N]? "
                .truecolor(255, 171, 0)
                .bold()
        );
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut continue_adding_platform)
            .expect("Error reading from STDIN");
        // remove carriage return
        continue_adding_platform.pop();

        match &continue_adding_platform.eq("y") {
            true => (),
            false => add_platform += 1,
        }
    }
    println!(
        "{}: {}",
        "The following client has been added".green().bold(),
        client_name
    );
}
