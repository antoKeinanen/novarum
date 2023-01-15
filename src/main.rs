use std::{
    env,
    fs::{File},
    io::{BufRead, BufReader},
    path::Path,
};

use dialoguer::{theme::SimpleTheme, FuzzySelect, MultiSelect, Select};

use platform_dirs::AppDirs;
use run_shell::cmd;
use setup::{generate_files, select_config};

mod setup;

#[derive(PartialEq)]
enum Mode {
    Basic,
    Select,
    MultiSelect,
    SearchSelect,
    If(bool),
}

fn main() {
    let app_dirs = AppDirs::new(Some("novarum"), false).unwrap();
    generate_files(&app_dirs);

    let file;
    if cfg!(debug_assertions) {
        file = File::open("configs/example.novconf").unwrap();
    } else {
        file = select_config(&app_dirs);
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
