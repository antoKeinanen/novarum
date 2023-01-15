use std::{
    collections::HashMap,
    env,
    fs::File,
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

    let mut name = String::new();
    let mut message = String::from("Select");

    let mut select_options: Vec<String> = vec![];
    let mut selected: HashMap<String, String> = HashMap::new();

    let mut multiselect_options: Vec<String> = vec![];
    let mut multi_selected: HashMap<String, Vec<String>> = HashMap::new();

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
                name = rest_of_line.to_string();

                if name.trim().len() < 1 {
                    panic!("Invalid name for select block on line {}", index + 1)
                }

                mode = Mode::Select;
            }
            "multiselect" => {
                name = rest_of_line.to_string();

                if name.trim().len() < 1 {
                    panic!("Invalid name for multiselect block on line {}", index + 1)
                }

                mode = Mode::MultiSelect;
            }
            "searchselect" => {
                name = rest_of_line.to_string();

                if name.trim().len() < 1 {
                    panic!("Invalid name for searchselect block on line {}", index + 1)
                }

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
                        let selected_option = Select::with_theme(&SimpleTheme)
                            .with_prompt(&message)
                            .items(&select_options)
                            .default(0)
                            .interact()
                            .unwrap();

                        selected.insert(name, format!("{}", select_options[selected_option]));
                        name = String::new();
                        select_options.clear();
                        message = String::from("Select");
                    }
                    Mode::MultiSelect => {
                        let selected_options = MultiSelect::with_theme(&SimpleTheme)
                            .with_prompt(&message)
                            .items(&multiselect_options)
                            .interact()
                            .unwrap();

                        let selected_options: Vec<String> = selected_options
                            .into_iter()
                            .map(|item| format!("{}", multiselect_options[item]))
                            .collect();

                        multi_selected.insert(name, selected_options);

                        name = String::new();
                        multiselect_options.clear();
                        message = String::from("Select");
                    }
                    Mode::SearchSelect => {
                        let selected_option = FuzzySelect::with_theme(&SimpleTheme)
                            .with_prompt(&message)
                            .items(&select_options)
                            .default(0)
                            .interact()
                            .unwrap();

                        selected.insert(name, format!("{}", select_options[selected_option]));
                        name = String::new();
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
                let if_name = rest_of_line.split_ascii_whitespace().collect::<Vec<&str>>()[0];
                let target = rest_of_line.trim_start_matches(if_name).trim_start();

                let multi_select_selection = multi_selected
                    .get(if_name)
                    .unwrap_or(&Vec::<String>::new())
                    .to_owned();

                for i in 0..multi_select_selection.len() {
                    let selection = format!("{}", multi_select_selection[i]);
                    if selection == target {
                        found = true;
                        break;
                    }
                }

                if selected.get(if_name).unwrap_or(&String::new()) == target || found {
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
