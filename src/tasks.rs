use chrono::{DateTime, Local, serde::ts_seconds, Utc};
use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};
use std::path::PathBuf;

/*
 * We won't add a status or is_complete field because we'll represent the to-do list as a vector
 * of tasks (Vec<Task>). So when a task is complete, we can simply remove it from the vector.
 **************************************************************************************************/
#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let created_at: DateTime<Utc> = Utc::now();
        Task { text, created_at }
    }
}

fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
    // Rewind the file before.
    file.seek(SeekFrom::Start(0))?;

    let tasks = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        // serde_json will return an error when it reaches the end of a file without having found
        // anything to parse. This event will always happen in an empty file, and we need to be
        // able to recover from it. Example in next line
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };

    // Rewind the file after.
    file.seek(SeekFrom::Start(0))?;

    Ok(tasks)
}

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    // Open the file.
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    // Consume the file's contents as a vetcor of tasks.
    let mut tasks = collect_tasks(&file)?;

    // Write the modified task list back into the file.
    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {
    // Open the file.
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    // Consume file's contents as a vector of tasks.
    let mut tasks = collect_tasks(&file)?;

    // Try to remove the task.
    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }
    tasks.remove(task_position - 1);

    // Write the modified task list back into the file.
    file.set_len(0)?;
    // We're truncating the file before writing to it because we're performing a removal operation.
    // So the file will be smaller than the original. If we ignored this step, the rewound cursor
    // would stop behind the previously written bytes of the file, resulting in a malformed
    // JSON file. When we truncate the file by using the file.set_len(0) operation,
    // we ensure that we're writing the bytes in a blank page.

    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn list_tasks(journal_path: PathBuf) -> Result<()> {
    // Open the file.
    let file = OpenOptions::new().read(true).open(journal_path)?;

    // Parse the file and collect the tasks.
    let tasks = collect_tasks(&file)?;

    // Enumerate and display tasks, if any.
    if tasks.is_empty() {
        println!("Task list is empty!");
    } else {
        let mut order: u32 = 1;
        for task in tasks {
            println!("{}: {}", order, task);
            // println!("{}: {:<50} [{}]", order, task.text, task.created_at.with_timezone(&Local).format("%F %H:%M"));
            order += 1;
        }
    }

    Ok(())
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        // https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html
        write!(f, "{:<50} [{}]", self.text, created_at)
    }
}
