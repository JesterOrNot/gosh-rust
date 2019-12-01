use colored::*;
fn main() {
    shell();
}
fn shell() {
    let mut rl = rustyline::Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(&prompt());
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}
fn command_not_found(command: &str) {
    let str_len = command.len();
    let highlight_thing = "^".repeat(str_len);
    println!(
        "{}{}{}{}\n{}          {}  {}",
        "1 | ".blue(),
        "❌  gosh ".red(),
        "--> ".blue(),
        command,
        "  |   ".blue(),
        highlight_thing.red(),
        "command not found".red()
    );
}
fn directory_not_found(dir: &str) {
    let str_len = dir.len();
    let highlight_thing = "^".repeat(str_len);
    println!(
        "{}{}{}{}{}\n{}          {}  {}",
        "1 | ".blue(),
        "❌  gosh ".red(),
        "--> ".blue(),
        "cd ",
        dir,
        "  |      ".blue(),
        highlight_thing.red(),
        "directory not found".red()
    );
}
fn pipe_error(command: String) {
    //WIP
}
fn prompt() -> String {
    let x = format!("{} {} ", "gosh".green(), "λ".blue());
    return x;
}
fn unique() {
    // WIP
}
fn completer() {
    // WIP
}
fn ls(directory: String) {
    // WIP
}
fn redirect_to_file() {
    // WIP
}
fn pipe_to_other_command() {
    // WIP
}
fn split_command_file() {
    // WIP
}
fn capture_output(command: String) {
    // WIP
}
fn split_commands() {
    // WIP
}
fn history() {
    // WIP
}
fn clear_history() {
    // WIP
}
fn update_history() {
    // WIP
}
fn help() {
    // WIP
}
fn execute_command() {
    // WIP
}
fn get_arg() {
    // WIP
}
fn get_command_hist() {
    // WIP
}
