use chrono::{Duration, Utc};
use clap::{Args, Parser, Subcommand};

use crate::database::TodoData;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub options: OptionTypes,
}

#[derive(Debug, Subcommand)]
pub enum OptionTypes {
    /// Add task, update task, view task
    Task(TaskOptions),
}

#[derive(Debug, Args)]
pub struct TaskOptions {
    #[command(subcommand)]
    pub command: TaskSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum TaskSubcommand {
    /// Add a new task
    Add(AddTask),

    /// Update a task
    Update(UpdateTask),

    /// View tasks
    View(ViewTask),
    
    /// Archive a task
    Archive(ArchiveTask),
    
    /// View archived tasks
    ViewArchive(ViewArchiveTask),
    
    Stats(GetStats),
}

#[derive(Debug, Args)]
pub struct AddTask {
    /// Project name
    #[arg(short, long, default_value_t = String::from("General"))]
    pub project: String,

    #[arg(short, long)]
    /// Task description
    pub task: String,

    #[arg(short, long, default_value_t = format!("{}", (Utc::now() + Duration::days(7)).format("%Y-%m-%d")))]
    /// Due date in format 'YYYY-MM-DD'
    pub due_date: String,

    /// Status
    #[arg(short, long, default_value_t = false)]
    pub complete: bool,
}

impl AddTask {
    #[must_use]
    pub fn to_todo_data(self) -> TodoData {
        TodoData {
            project: self.project,
            task: self.task,
            due_date: self.due_date,
            complete: self.complete,
        }
    }
}

#[derive(Debug, Args, Copy, Clone)]
pub struct UpdateTask {
    /// Row ID for task
    pub id: u64,

    /// Mark as complete (no args needed just the flag i.e -c or --complete)
    #[arg(short, long, default_value_t = false)]
    pub complete: bool,

    /// Delete task (no args needed just the flag i.e -d or --delete)
    #[arg(short, long, default_value_t = false)]
    pub delete: bool,
}

impl UpdateTask {
    #[must_use]
    pub fn to_todo_data(self) -> TodoData {
        TodoData {
            project: String::from("Placeholder"),
            task: String::from("Placeholder"),
            due_date: String::from("Placeholder"),
            complete: self.complete,
        }
    }
}

#[derive(Debug, Args)]
pub struct ViewTask {
    /// View specific project
    #[arg(short, long, default_value_t = String::from("All"))]
    pub project: String,
}

#[derive(Debug, Args)]
pub struct ArchiveTask {
    /// Row ID for task to archive
    pub id: u64,
}

#[derive(Debug, Args)]
pub struct ViewArchiveTask {
    /// View archived tasks for specific project
    #[arg(short, long, default_value_t = String::from("All"))]
    pub project: String,
}

#[derive(Debug, Args)]
pub struct GetStats {
    /// Get pending tasks count (no args needed just the flag i.e -p or --pending)
    #[arg(short, long, default_value_t = false)]
    pub pending: bool,

    /// Get overdue tasks count (no args needed just the flag i.e -o or -overdue)
    #[arg(short, long, default_value_t = false)]
    pub overdue: bool,
}
