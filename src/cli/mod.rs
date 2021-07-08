use clap::{crate_authors, crate_version, App, SubCommand};

pub struct Config {
    pub run: bool,
}

pub fn check_cli() -> Config {
    // Create clap app and get matches.
    let matches = App::new("Config Center")
        .about("Easily config any app!")
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(SubCommand::with_name("run").about("Lunch GUI."))
        .get_matches();

    // Get run state from matches.
    let run = if let Some(_) = matches.subcommand_matches("run") {
        true
    } else {
        false
    };

    // Create and return config.
    return Config { run };
}
