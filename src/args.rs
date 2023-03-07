use clap::{Args, Parser, Subcommand};
use todo::DEFAULT_DATE;

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
    pub command: TaskSubcommand
}

#[derive(Debug, Subcommand)]
pub enum TaskSubcommand {
    /// Add a new task
    Add(AddTask),

    /// Update a task
    Update(UpdateTask),

    /// View tasks
    View(ViewTask),
}

#[derive(Debug, Args)]
pub struct AddTask {
    /// Project name
    #[arg(default_value_t = String::from("General"))]
    project: String,

    #[arg(short, long)]
    /// Task description
    task: String,

    #[arg(short, long, default_value_t = format!("{}", DEFAULT_DATE.format("%Y-%m-%d")))]
    /// Due date in format 'YYYY-MM-DD'
    due_date: String
}

#[derive(Debug, Args)]
pub struct UpdateTask {
    /// Mark as complete
    #[arg(default_value_t = false)]
    complete: bool,

    /// Delete task
    #[arg(default_value_t = false)]
    delete: bool,
}

#[derive(Debug, Args)]
pub struct ViewTask {
    /// View specific project
    #[arg(default_value_t = String::from("All"))]
    project: String,
}