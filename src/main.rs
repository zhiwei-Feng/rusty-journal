mod cli;
mod tasks;

use std::path::PathBuf;

use anyhow::anyhow;
use cli::{Action::*, CommandLineArgs};
use structopt::StructOpt;
use tasks::Task;

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        println!("default journal file {:?}", path);
        path
    })
}

/// 1. structopt 用于处理命令行参数，并绑定至结构体中
/// 2. serde_json 用于序列化和反序列化
/// 3. anyhow 友好的错误输出
fn main() -> anyhow::Result<()> {
    // Get the command-line arguments.
    let CommandLineArgs {
        action,
        journal_file,
    } = cli::CommandLineArgs::from_args();

    // Unpack the journal file.
    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .ok_or(anyhow!("Failed to find journal file."))?;

    // Perform the action.
    match action {
        Add { task } => tasks::add_task(journal_file, Task::new(task)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
    }?;
    // println!("Hello, world!");
    Ok(())
}
