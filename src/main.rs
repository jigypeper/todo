use chrono::{DateTime, Utc};
use todo::database::TodoData;

fn main() {
    let mock_data = TodoData {
        id: 5,
        project: String::from("test"),
        task: String::from("mangoes are amazing"),
        due_data: Utc::now(),
        complete: true,
    };

    mock_data.write_data();
}
