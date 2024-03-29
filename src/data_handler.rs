use crate::{
    args::{TaskSubcommand, UpdateTask},
    database::{count_overdue, count_pending, get_all_tasks, get_tasks},
    views::show_data,
};

static DB_FILE: &str = "todo.db";

pub fn handle_data(data_to_handle: TaskSubcommand) {
    let path = std::env::current_exe().unwrap();
    let dir = path.parent().expect("Binary should be in a directory");

    match data_to_handle {
        TaskSubcommand::Add(task) => {
            let new_task = task.to_todo_data();
            new_task
                .write_data(dir.join(DB_FILE).to_str().unwrap())
                .expect("No data");
        }
        TaskSubcommand::Update(task) => {
            let parameters: UpdateTask = UpdateTask {
                id: (task.id),
                complete: (task.complete),
                delete: (task.delete),
            };
            let new_task = task.to_todo_data();
            new_task
                .update_task(parameters, dir.join(DB_FILE).to_str().unwrap())
                .expect("Database does not exist, create task first");
        }
        TaskSubcommand::View(view) => match &view.project[..] {
            "All" => {
                let results = get_all_tasks(dir.join(DB_FILE).to_str().unwrap());
                match results {
                    Ok(data) => {
                        let output = show_data(data);
                        output.printstd();
                    }
                    Err(_) => eprintln!("No database or data"),
                }
            }
            _ => {
                let results = get_tasks(&view.project[..], dir.join(DB_FILE).to_str().unwrap());
                match results {
                    Ok(data) => {
                        let output = show_data(data);
                        output.printstd();
                    }
                    Err(_) => eprintln!("No database or data"),
                }
            }
        },
        TaskSubcommand::Stats(numbers) => {
            if numbers.pending && numbers.overdue {
                println!(
                    "{}",
                    count_pending(dir.join(DB_FILE).to_str().unwrap()).unwrap()
                )
            } else if !numbers.pending && numbers.overdue {
                println!(
                    "{}",
                    count_overdue(dir.join(DB_FILE).to_str().unwrap()).unwrap()
                )
            } else {
                println!(
                    "{}",
                    count_pending(dir.join(DB_FILE).to_str().unwrap()).unwrap()
                )
            }
        }
    }
}
