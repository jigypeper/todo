use crate::args::{TaskSubcommand, UpdateTask};

pub fn handle_data(data_to_handle: TaskSubcommand) {
    match data_to_handle {
        TaskSubcommand::Add(task) => {
            // TODO: find way to change AddTask struct to TodoData struct
            // let new_task = TodoData {
            //     ..task
            // };

            // new_task.write_data();
            let new_task = task.to_todo_data();
            new_task.write_data().expect("No data");
        },
        TaskSubcommand::Update(task) => {
            let parameters: UpdateTask = UpdateTask { 
                id: (task.id.clone()), 
                complete: (task.complete.clone()), 
                delete: (task.delete.clone()) 
            };
            let new_task = task.to_todo_data();
            new_task.update_task(parameters).expect("Database does not exist, create task first");
        },
        TaskSubcommand::View(_) => todo!(),
    }
}
 