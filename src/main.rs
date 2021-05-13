use structopt::StructOpt;
use cli::{Action::*, CommandLineArgs};
use tasks::Task;
use std::path::PathBuf;

mod cli;
mod tasks;


fn find_default_journal_file() -> Option<PathBuf> {
    // Because home directories vary depending on the user's operating system,
    // we'll rely on a third-party crate called home to determine the directory.
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}

fn main() {
    // Get the command-line arguments.
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    // Unpack the journal file.
    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .expect("Failed to find journal file.");

    // Perform the action.
    match action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),

    }.expect("Failed to perform action")

    // println!("{:#?}", cli::CommandLineArgs::from_args());
}