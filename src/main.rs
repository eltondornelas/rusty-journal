use anyhow::anyhow;
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

fn main() -> anyhow::Result<()> {
    // anyhow is an excellent crate for displaying useful and pretty errors to users
    // Because most error types can be converted to anyhow::Error, we can use ? syntax to remove
    // the expect calls from our code. Also, note that we're using the anyhow! macro to
    // produce an anyhow::Error on the fly that contains the provided error message.

    // Get the command-line arguments.
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    // Unpack the journal file.
    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .ok_or(anyhow!("Failed to find journal file."))?;

    // Perform the action.
    match action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
    }?;

    Ok(())
    // println!("{:#?}", cli::CommandLineArgs::from_args());
}