// use chrono::{DateTime, Utc};
use clap::Parser;
// use todo::database::TodoData;
use todo::{args::{Cli, OptionTypes, TaskSubcommand}, data_handler};

fn main() {
    let cli = Cli::parse();

    println!("{:?}", cli);

    match cli.options {
        OptionTypes::Task(more_options) => data_handler::handle_data(more_options.command)
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
