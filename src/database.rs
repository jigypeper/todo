use rusqlite::{Connection, Result, named_params};


use crate::{args::UpdateTask};
// use chrono::{DateTime, Utc};

pub trait Viewer {}


pub struct TodoData {
    pub project: String,
    pub task: String,
    pub due_date: String,
    pub complete: bool,
}


impl TodoData {
    pub fn write_data(self, db_file: &str) -> Result<()> {
        let mut conn = Connection::open(db_file).unwrap();
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS data (
                id INTEGER PRIMARY KEY NOT NULL, 
                project VARCHAR(50) NOT NULL,
                task VARCHAR(100) NOT NULL,
                due_date DATE,
                complete BOOLEAN NOT NULL CHECK (complete IN (0, 1))
            );", 
            (),
        )?;
        
        let tx = conn.transaction()?;
        tx.execute(
            "INSERT OR REPLACE INTO data (project, task, due_date, complete)
            VALUES (:project, :task, :due_date, :complete)",
            named_params! {
                ":project": self.project,
                ":task": self.task,
                ":due_date": self.due_date,
                ":complete": match self.complete {
                    true => 1,
                    false => 0,
                }, 
            }
        )?;

        tx.commit()?;

        Ok(())
    }

    pub fn update_task(self, update_task: UpdateTask, db_file: &str) -> Result <()> {
        if update_task.complete == true && update_task.delete == true {
            println!("Cannot delete and update a task");
            Ok(())
        } else if update_task.complete == true {
            let mut conn = Connection::open(db_file).unwrap();
        
        
            let tx = conn.transaction()?;
            tx.execute(
                "UPDATE data
                SET complete = :complete
                WHERE id = :id",
                named_params! {
                    ":id": update_task.id,
                    ":complete": match self.complete {
                        true => 1,
                        false => 0,
                    }, 
                }
            )?;

            tx.commit()?;

            Ok(())
        } else {
            let mut conn = Connection::open(db_file).unwrap();
        
        
            let tx = conn.transaction()?;
            tx.execute(
                "DELETE FROM data
                WHERE id = :id",
                named_params! {
                    ":id": update_task.id,
                }
            )?;

            tx.commit()?;

            Ok(())
        }
    }

    pub fn get_all(self) -> Option<Vec<TodoData>> {
        todo!();
    }

}

pub struct TodoView {
    pub project: u64,
    pub task: String,
    pub due_date: String,
    pub complete: bool,
}

impl Viewer for TodoView {}

pub fn get_tasks(project_name: &str, db_file: &str) -> Result<Vec<TodoView>> {
    let mut conn = Connection::open(db_file).unwrap();
    
    let mut stmt = match project_name {
        "All" => conn.prepare(
                "SELECT * FROM data;",
            )?,

        _ => conn.prepare(
                "SELECT * FROM data
                 WHERE project = :project_name;",
            )?,
    };
        

    let tasks_iter = stmt.query_map([], |row| {
        Ok(TodoView {
            project: row.get(0)?,
            task: row.get(1)?,
            due_date: row.get(2)?,
            complete: match row.get(3).unwrap() {
                1 => true,
                0 => false,
                _ => false
            }
        })
    })?;

    let mut result = Vec::new();

    for task in tasks_iter {
        result.push(task?);
    }

    Ok(result)
    
}


#[cfg(test)]
mod tests {
    use super::*;

    static test_database: &str = "test.db";

    #[test]
    fn add_data() {
        let sample = TodoData {
            project: String::from("Test"),
            task: String::from("Test"),
            due_date: String::from("2023-01-01"),
            complete: false,
        };

        assert_eq!(Ok(()), sample.write_data(test_database));
    }

    #[test]
    fn update_data() {
        let prepare = TodoData {
            project: String::from("Test"),
            task: String::from("Test"),
            due_date: String::from("2023-01-01"),
            complete: false,
        };

        let sample = TodoData {
            project: String::from("Test"),
            task: String::from("Test"),
            due_date: String::from("2023-01-01"),
            complete: false,
        };

        prepare.write_data(test_database).expect("database does not exist");

        let sample_param = UpdateTask { 
            id: 1, 
            complete: true, 
            delete: false
        };

        assert_eq!(Ok(()), sample.update_task(sample_param, test_database));

    }

    #[test]
    fn delete_data() {
        let prepare = TodoData {
            project: String::from("Test"),
            task: String::from("Test"),
            due_date: String::from("2023-01-01"),
            complete: false,
        };

        let sample = TodoData {
            project: String::from("Test"),
            task: String::from("Test"),
            due_date: String::from("2023-01-01"),
            complete: false,
        };

        prepare.write_data(test_database).expect("database does not exist");

        let sample_param = UpdateTask { 
            id: 1, 
            complete: false, 
            delete: true
        };

        assert_eq!(Ok(()), sample.update_task(sample_param, test_database));
    }

}