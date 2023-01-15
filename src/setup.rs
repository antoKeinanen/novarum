use std::{
    fs::{create_dir_all, File},
    io::Write,
};

use dialoguer::{Confirm, theme::SimpleTheme, FuzzySelect};
use platform_dirs::AppDirs;

pub fn generate_files(app_dirs: &AppDirs) {
    let example_bytes = include_bytes!("../configs/example.novconf");

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
}

pub fn select_config(app_dirs: &AppDirs) -> File {
    let mut configs: Vec<String> = vec![];
    let mut config_paths: Vec<String> = vec![];

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

    File::open(&config_paths[selected_config]).unwrap()
}
