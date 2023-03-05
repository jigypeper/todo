use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(Subcommand)]
    pub options: OptionTypes,
}

#[derive(Debug, Subcommand)]
pub enum OptionTypes {
    /// Add task, update task, view task
    Task(TaskOptions),
}

#[derive(Debug, Args)]
pub struct TaskOptions {
    #[clap(Subcommand)]
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
    #[clap(default_value_t = String::from("General"))]
    project: String,

    /// Task description
    task: String,

    /// Due date in format 'YYYY-MM-DD'
    due_date: String
}

#[derive(Debug, Args)]
pub struct UpdateTask {
    /// Mark as complete
    #[clap(default_value_t = false)]
    complete: bool,

    /// Delete task
    #[clap(default_value_t = false)]
    delete: bool,
}

#[derive(Debug, Args)]
pub struct ViewTask {
    /// View all
    #[clap(default_value_t = false)]
    all: bool,

    /// View specific project
    project: String,
}