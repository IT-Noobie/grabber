use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::ContentArrangement;
use comfy_table::{Cell, Row};
use std::fs::File;
use std::io::{Error, Read};
use std::process::exit;
use toml::value::Table;
use toml::Value;

pub fn platforms() -> Result<(), Error> {
    let ssh_config_file_path: String = format!(
        "{}/.grabber/grabber-config.toml",
        dirs::home_dir().unwrap().display()
    );

    let mut file = File::open(ssh_config_file_path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    let mut table = comfy_table::Table::new();
    table
        .set_header(vec!["Platforms SSH Key alias"])
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic);
    let toml: Table = toml::from_str(&contents)?;
    for key in toml.keys() {
        let mut row: Row = Row::new();
        row.add_cell(Cell::new(key));
        table.add_row(row);
    }
    println!("{table}");
    Ok(())
}

pub fn clients() -> Result<(), Error> {
    let ssh_config_file_path: String = format!(
        "{}/.grabber/grabber-repositories.toml",
        dirs::home_dir().unwrap().display()
    );
    let mut file = File::open(ssh_config_file_path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    let mut table = comfy_table::Table::new();
    table
        .set_header(vec![format!("CLIENTS")])
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic);

    let toml: Table = toml::from_str(&contents)?;
    //let keys = toml.into.collect::<Value>;
    let clients = toml.keys();
    for key in clients {
        let mut row: Row = Row::new();
        row.add_cell(Cell::new(key));
        table.add_row(row);
    }
    println!("{}", table);
    Ok(())
}

pub fn client_platform(client: &String) -> Result<(), Error> {
    let ssh_config_file_path: String = format!(
        "{}/.grabber/grabber-repositories.toml",
        dirs::home_dir().unwrap().display()
    );
    let mut file = File::open(ssh_config_file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let toml: Value = toml::from_str(&contents)?;
    let mut table = comfy_table::Table::new();
    table
        .set_header(vec![format!("{} PLATFORMS", &client.to_ascii_uppercase())])
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic);

    match toml.get(client) {
        None => {
            eprintln!("ERROR: Doesn't exist client {}", &client);
            exit(1)
        }
        Some(_) => {
            match toml[client].as_table() {
                None => eprintln!("ERROR: Unable to convert to TOML data as a table"),
                Some(inner_table) => {
                    for key in inner_table.keys() {
                        let mut row: Row = Row::new();
                        row.add_cell(Cell::new(key));
                        table.add_row(row);
                    }
                    println!("{}", table);
                }
            };
        }
    };
    Ok(())
}

pub fn platform_key_alias_config(platform_key_alias: &String) -> Result<(), Error> {
    let ssh_config_file_path: String = format!(
        "{}/.grabber/grabber-config.toml",
        dirs::home_dir().unwrap().display()
    );

    let mut file = File::open(ssh_config_file_path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let toml: Table = toml::from_str(&contents)?;
    match toml.get(platform_key_alias) {
        None => eprintln!("ERROR: SSH key configuration not found for: {}\nRun 'grabber list' to show a list of all configured keys.", platform_key_alias),
        Some(key) => {
            let mut table = comfy_table::Table::new();
            table
                .set_header(vec![&platform_key_alias.to_ascii_uppercase()])
                .load_preset(UTF8_FULL)
                .apply_modifier(UTF8_ROUND_CORNERS)
                .set_content_arrangement(ContentArrangement::Dynamic);
            let mut row: Row = Row::new();
            row.add_cell(Cell::new(key));
            table.add_row(row);
            println!("{}", table);
        },
    }
    Ok(())
}

pub fn client_platform_repositories(client: &String, platform: &String) -> Result<(), Error> {
    let ssh_config_file_path: String = format!(
        "{}/.grabber/grabber-repositories.toml",
        dirs::home_dir().unwrap().display()
    );

    let mut file = File::open(ssh_config_file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let toml: Value = toml::from_str(&contents)?;

    match toml.get(client) {
        None => eprintln!("ERROR: SSH key configuration not found for: {}\nRun 'grabber list' to show a list of all configured keys.", client),
        Some(client_id) => {
            match client_id.get(platform) {
                None => eprintln!("ERROR: Platform not found"),
                Some(platform_key_alias) => {
                    let mut table = comfy_table::Table::new();
                    table
                        .set_header(vec![format!("{} {} REPOSITORIES", &client.to_ascii_uppercase(), &platform.to_ascii_uppercase())])
                        .load_preset(UTF8_FULL)
                        .apply_modifier(UTF8_ROUND_CORNERS)
                        .set_content_arrangement(ContentArrangement::Dynamic);
                    let inner_table = platform_key_alias["repositories"].as_array().unwrap();
                    for value in inner_table {
                        let mut row: Row = Row::new();
                        row.add_cell(Cell::new(value));
                        table.add_row(row);
                    }
                    println!("{}", table);
                },
            };
        },
    };
    Ok(())
}
