use crate::args::TaskSubcommand;

pub fn add_data(data_to_handle: TaskSubcommand) {
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
        TaskSubcommand::Update(_) => todo!(),
        TaskSubcommand::View(_) => todo!(),
    }
} 