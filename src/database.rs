use rusqlite::{params, Connection, Result, named_params};
use chrono::{DateTime, Utc};

pub struct TodoData {
    pub id: u64,
    pub project: String,
    pub task: String,
    pub due_data: DateTime<Utc>,
    pub complete: bool,
}

impl TodoData {
    pub fn write_data(self) -> Result<()>{
        let mut conn = Connection::open("todo.db").unwrap();
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS data (
                id INT PRIMARY KEY, 
                project VARCHAR(50) NOT NULL,
                task VARCHAR(100) NOT NULL,
                due_date DATE,
                complete BOOLEAN NOT NULL CHECK (complete IN (0, 1))
            );", 
            (),
        )?;

        let tx = conn.transaction()?;
        tx.execute(
            "INSERT OR REPLACE INTO data (id, project, task, due_date, complete)
            VALUES (:id, :project, :task, :due_date, :complete)",
            named_params! {
                ":id": self.id,
                ":project": self.project,
                ":task": self.task,
                ":due_date": format!("{}", self.due_data.format("%Y-%m-%d")),
                ":complete": match self.complete {
                    true => 1,
                    false => 0,
                }, 
            }
        )?;

        tx.commit()?;

        Ok(())
    }

    pub fn get_all(self) -> Option<Vec<TodoData>> {
        todo!();
    } 
}