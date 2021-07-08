mod cli;
mod gui;

fn main() {
    // check cli
    let config = cli::check_cli();

    // check plugins
    // run gui
    // edit files
    if config.run {
        gui::lunch_gui();
    }
}
