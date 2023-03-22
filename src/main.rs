use clap::Parser;
use todo::{
    args::{Cli, OptionTypes},
    data_handler,
};

fn main() {
    let cli = Cli::parse();

    // println!("{:?}", cli);

    match cli.options {
        OptionTypes::Task(more_options) => data_handler::handle_data(more_options.command),
    }
}
