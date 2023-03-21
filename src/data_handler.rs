use crate::args::{TaskSubcommand, UpdateTask};

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
        TaskSubcommand::View(_) => todo!(),
    }
}
