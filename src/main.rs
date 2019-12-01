use colored::*;
use rustyline_derive::Helper;
use std::borrow::Cow;
use std::borrow::Cow::Borrowed;
use std::borrow::Cow::Owned;

fn main() {
    shell();
}
#[derive(Helper)]
struct Helper {
    completer: rustyline::completion::FilenameCompleter,
    highlighter: rustyline::highlight::MatchingBracketHighlighter,
    hinter: rustyline::hint::HistoryHinter,
    colored_prompt: String,
}
impl rustyline::completion::Completer for Helper {
    type Candidate = rustyline::completion::Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &rustyline::Context<'_>,
    ) -> Result<(usize, Vec<rustyline::completion::Pair>), rustyline::error::ReadlineError> {
        self.completer.complete(line, pos, ctx)
    }
}

impl rustyline::hint::Hinter for Helper {
    fn hint(&self, line: &str, pos: usize, ctx: &rustyline::Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl rustyline::highlight::Highlighter for Helper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Borrowed(&self.colored_prompt)
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.highlighter.highlight_char(line, pos)
    }
}
fn shell() {
    env_logger::init();
    let config = rustyline::Config::builder()
        .history_ignore_space(true)
        .completion_type(rustyline::CompletionType::List)
        .edit_mode(rustyline::EditMode::Emacs)
        .output_stream(rustyline::OutputStreamType::Stdout)
        .build();
    let h = Helper {
        completer: rustyline::completion::FilenameCompleter::new(),
        highlighter: rustyline::highlight::MatchingBracketHighlighter::new(),
        hinter: rustyline::hint::HistoryHinter {},
        colored_prompt: "".to_owned(),
    };
    let mut rl = rustyline::Editor::with_config(config);
    rl.set_helper(Some(h));
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        rl.helper_mut().expect("No helper").colored_prompt = prompt();
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
