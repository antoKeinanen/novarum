use std::{
    env,
    fs::{create_dir_all, File},
    io::{BufRead, BufReader, Write},
    path::Path,
};

use dialoguer::{theme::SimpleTheme, Confirm, FuzzySelect, MultiSelect, Select};

use platform_dirs::AppDirs;
use run_shell::cmd;

#[derive(PartialEq)]
enum Mode {
    Basic,
    Select,
    MultiSelect,
    SearchSelect,
    If(bool),
}

fn main() {
    let example_bytes = include_bytes!("../configs/example.novconf");

    let app_dirs = AppDirs::new(Some("novarum"), false).unwrap();

    if !app_dirs.config_dir.exists() {
        println!("It looks like you are missing config directory at:");
        println!("{:?}", &app_dirs.config_dir);
        let confirmed = Confirm::new()
            .with_prompt("Would you like to create one now?")
            .interact()
            .unwrap();

        if confirmed {
            println!("Creating config folder...");
            create_dir_all(&app_dirs.config_dir).expect("Failed to create config folder!");
            println!("Writing example config to config folder...");

            let mut file = File::create(app_dirs.config_dir.join("example.novconf"))
                .expect("Failed to create example config file!");
            file.write(example_bytes)
                .expect("Failed to write example config!");
        } else {
            println!("Exiting...");
            return;
        }
    }

    let mut configs: Vec<String> = vec![];
    let mut config_paths: Vec<String> = vec![];

    let file;
    if cfg!(debug_assertions) {
        file = File::open("configs/example.novconf").unwrap();
    } else {
        for entry in glob::glob(app_dirs.config_dir.join("*.novconf").to_str().unwrap())
            .expect("Failed to read glob pattern!")
        {
            match entry {
                Ok(path) => {
                    configs.push(
                        path.as_path()
                            .file_stem()
                            .unwrap()
                            .to_string_lossy()
                            .to_string(),
                    );
                    config_paths.push(path.to_str().unwrap().to_string());
                }
                Err(e) => println!("{:?}", e),
            }
        }

        let selected_config = FuzzySelect::with_theme(&SimpleTheme)
            .with_prompt("Select config to be used (type to search):")
            .items(&configs)
            .default(0)
            .interact()
            .unwrap();

        file = File::open(&config_paths[selected_config]).unwrap();
    }

    let reader = BufReader::new(file);

    let mut select_options: Vec<String> = vec![];
    let mut last_selected: String = String::new();

    let mut multiselect_options: Vec<String> = vec![];
    let mut last_multiselected: Vec<String> = vec![];

    let mut message = String::from("Select");

    let mut mode: Mode = Mode::Basic;

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        if line.len() == 0 {
            continue;
        }

        let line = line.trim_start().replace("\n", "");

        let first_token: &str = line.split_ascii_whitespace().collect::<Vec<&str>>()[0];
        let rest_of_line = line.trim_start_matches(first_token).trim_start();

        if line.starts_with("#") || mode == Mode::If(false) && first_token != "end" {
            continue;
        }

        match first_token {
            "-" => {
                if mode == Mode::Select || mode == Mode::SearchSelect {
                    select_options.push(rest_of_line.to_owned())
                } else if mode == Mode::MultiSelect {
                    multiselect_options.push(rest_of_line.to_owned())
                } else {
                    panic!(
                        "List operator used in other than Select or MultiSelectMode at line {}",
                        index + 1
                    )
                }
            }
            "shell" => {
                cmd!(rest_of_line)
                    .run()
                    .expect(format!("Failed to run command on line {}", index + 1).as_str());
            }
            "select" => {
                mode = Mode::Select;
            }
            "multiselect" => {
                mode = Mode::MultiSelect;
            }
            "searchselect" => {
                mode = Mode::SearchSelect;
            }
            "print" => {
                println!("{}", rest_of_line);
            }
            "message" => {
                message = rest_of_line.to_string();
            }
            "end" => {
                match mode {
                    Mode::Select => {
                        let selected = Select::with_theme(&SimpleTheme)
                            .with_prompt(&message)
                            .items(&select_options)
                            .default(0)
                            .interact()
                            .unwrap();

                        last_selected = format!("{}", select_options[selected]);
                        select_options.clear();
                        message = String::from("Select");
                    }
                    Mode::MultiSelect => {
                        let selectees = MultiSelect::with_theme(&SimpleTheme)
                            .with_prompt(&message)
                            .items(&multiselect_options)
                            .interact()
                            .unwrap();

                        for selected in selectees {
                            last_multiselected.push(format!("{}", multiselect_options[selected]))
                        }

                        multiselect_options.clear();
                        message = String::from("Select");
                    }
                    Mode::SearchSelect => {
                        let selected = FuzzySelect::with_theme(&SimpleTheme)
                            .with_prompt(&message)
                            .items(&select_options)
                            .default(0)
                            .interact()
                            .unwrap();

                        last_selected = format!("{}", select_options[selected]);
                        select_options.clear();
                        message = String::from("Select");
                    }
                    Mode::If(_) => {}

                    _ => panic!("Unexpected end token on line {}", index + 1),
                }
                mode = Mode::Basic;
            }
            "if" => {
                let mut found = false;

                for i in 0..last_multiselected.len() {
                    let selected = &last_multiselected[i];
                    if selected == rest_of_line {
                        found = true;
                        break;
                    }
                }

                if last_selected == rest_of_line || found {
                    mode = Mode::If(true);
                } else {
                    mode = Mode::If(false);
                }
            }
            "chdir" => {
                let path = Path::new(rest_of_line);
                env::set_current_dir(path)
                    .expect(format!("Failed to set working dir on line {}", index + 1).as_str());
            }
            _ => {
                panic!("Unknown token '{}' at line {}", first_token, index + 1)
            }
        }
    }
}
