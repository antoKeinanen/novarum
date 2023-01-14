use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::{self, Path},
};

use dialoguer::{theme::SimpleTheme, MultiSelect, Select};

use run_shell::cmd;

#[derive(PartialEq)]
enum Mode {
    Basic,
    Select,
    MultiSelect,
    If(bool),
}

fn main() {
    

    let file = File::open("configs/rust.novconf").unwrap();
    let reader = BufReader::new(file);

    let mut select_options: Vec<String> = vec![];
    let mut last_selected: String = String::new();

    let mut multiselect_options: Vec<String> = vec![];
    let mut last_multiselected: Vec<String> = vec![];

    let mut mode: Mode = Mode::Basic;

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        if line.len() == 0 {
            continue;
        }

        let line = line.trim_start();

        let first_token: &str = line.split_ascii_whitespace().collect::<Vec<&str>>()[0];
        let rest_of_line = line.trim_start_matches(first_token).trim_start();

        if line.starts_with("#") || mode == Mode::If(false) && first_token != "end" {
            continue;
        }

        match first_token {
            "-" => {
                if mode == Mode::Select {
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
            "message" => {
                println!("{}", rest_of_line);
            }
            "multiselect" => {
                mode = Mode::MultiSelect;
            }
            "end" => {
                match mode {
                    Mode::Select => {
                        let selected = Select::with_theme(&SimpleTheme)
                            .items(&select_options)
                            .default(0)
                            .interact()
                            .unwrap();

                        last_selected = format!("{}", select_options[selected]);
                        select_options.clear();
                    }
                    Mode::MultiSelect => {
                        let selectees = MultiSelect::with_theme(&SimpleTheme)
                            .items(&multiselect_options)
                            .interact()
                            .unwrap();

                        for selected in selectees {
                            last_multiselected.push(format!("{}", multiselect_options[selected]))
                        }

                        multiselect_options.clear();
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

/*
let items = vec!["Option1", "Option2"];
let selected = Select::with_theme(&SimpleTheme)
.items(&items)
.default(0)
.interact().unwrap();

println!("{:?}", selected) */
