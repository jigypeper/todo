// use chrono::{DateTime, Utc};
use clap::Parser;
// use todo::database::TodoData;
use todo::args::{Cli, OptionTypes, TaskSubcommand};

fn main() {
    let cli = Cli::parse();

    println!("{:?}", cli);

    match cli.options {
        OptionTypes::Task(more_options) => match more_options.command {
            TaskSubcommand::Add(task) => {
                // TODO: find way to change AddTask struct to TodoData struct
                // let new_task = TodoData {
                //     ..task
                // };

                // new_task.write_data();
                let new_task = task.to_todo_data();
                new_task.write_data().expect("No data");
            },
            _ => println!("Other stuff"),
        }
    }


}

// Data persistence testing
// let mock_data = TodoData {
//     id: 5,
//     project: String::from("test"),
//     task: String::from("mangoes are amazing"),
//     due_data: Utc::now(),
//     complete: true,
// };

// mock_data.write_data();
