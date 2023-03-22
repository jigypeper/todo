use crate::{
    args::{TaskSubcommand, UpdateTask},
    database::{get_all_tasks, get_tasks},
    views::show_data,
};

static DB_FILE: &str = "todo.db";

pub fn handle_data(data_to_handle: TaskSubcommand) {
    match data_to_handle {
        TaskSubcommand::Add(task) => {
            let new_task = task.to_todo_data();
            new_task.write_data(DB_FILE).expect("No data");
        }
        TaskSubcommand::Update(task) => {
            let parameters: UpdateTask = UpdateTask {
                id: (task.id.clone()),
                complete: (task.complete.clone()),
                delete: (task.delete.clone()),
            };
            let new_task = task.to_todo_data();
            new_task
                .update_task(parameters, DB_FILE)
                .expect("Database does not exist, create task first");
        }
        TaskSubcommand::View(view) => match &view.project[..] {
            "All" => {
                let results = get_all_tasks(DB_FILE);
                match results {
                    Ok(data) => {
                        let output = show_data(data);
                        output.printstd();
                    }
                    Err(_) => println!("No database or data"),
                }
            }
            _ => {
                let results = get_tasks(&view.project[..], DB_FILE);
                match results {
                    Ok(data) => {
                        let output = show_data(data);
                        output.printstd();
                    }
                    Err(_) => println!("No database or data"),
                }
            }
        },
    }
}
