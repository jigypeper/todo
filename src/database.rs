use crate::args::UpdateTask;
use rusqlite::{named_params, Connection, Result};

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
            },
        )?;

        tx.commit()?;

        Ok(())
    }

    pub fn update_task(self, update_task: UpdateTask, db_file: &str) -> Result<()> {
        if update_task.complete && update_task.delete == true {
            println!("Cannot delete and update a task");
            Ok(())
        } else if update_task.complete {
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
                },
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
                },
            )?;

            tx.commit()?;

            Ok(())
        }
    }
    
    pub fn archive(self) -> Result<()> {
        // is this a requirement?
        // perhaps needs to push to online db?
        // maybe needs an api as a microservice
        todo!();
    }

    pub fn set_reminder(self) -> Result<()> {
        todo!();
    }

}

#[derive(Debug, PartialEq)]
pub struct TodoView {
    pub id: u64,
    pub project: String,
    pub task: String,
    pub due_date: String,
    pub complete: bool,
}

pub fn get_tasks(project_name: &str, db_file: &str) -> Result<Vec<TodoView>> {
    let conn = Connection::open(db_file).unwrap();

    let mut stmt = conn.prepare(
        "SELECT * FROM data
                 WHERE project = :project_name OR :project_name IS NULL;",
    )?;

    // TODO: Need to match on this to get query binding version
    let tasks_iter = stmt.query_map(&[(":project_name", &project_name)], |row| {
        Ok(TodoView {
            id: row.get(0)?,
            project: row.get(1)?,
            task: row.get(2)?,
            due_date: row.get(3)?,
            complete: match row.get(4).unwrap() {
                1 => true,
                _ => false,
            },
        })
    })?;

    let mut result = Vec::new();

    for task in tasks_iter {
        result.push(task?);
    }

    Ok(result)
}

pub fn get_all_tasks(db_file: &str) -> Result<Vec<TodoView>> {
    let conn = Connection::open(db_file).unwrap();

    let mut stmt = conn.prepare("SELECT * FROM data;")?;

    let tasks_iter = stmt.query_map([], |row| {
        Ok(TodoView {
            id: row.get(0)?,
            project: row.get(1)?,
            task: row.get(2)?,
            due_date: row.get(3)?,
            complete: match row.get(4).unwrap() {
                1 => true,
                _ => false,
            },
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

    static TEST_DATABASE: &str = "test.db";

    fn drop_table() -> Result<()> {
        let conn = Connection::open(TEST_DATABASE).unwrap();
        conn.execute("DROP TABLE IF EXISTS data;", ())?;

        Ok(())
    }

    #[test]
    fn add_data() {
        let sample = TodoData {
            project: String::from("Test"),
            task: String::from("Test"),
            due_date: String::from("2023-01-01"),
            complete: false,
        };

        assert_eq!(Ok(()), sample.write_data(TEST_DATABASE));

        drop_table().unwrap();
    }

    #[test]
    fn update_data() {
        drop_table().unwrap();

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

        prepare
            .write_data(TEST_DATABASE)
            .expect("database does not exist");

        let sample_param = UpdateTask {
            id: 1,
            complete: true,
            delete: false,
        };

        assert_eq!(Ok(()), sample.update_task(sample_param, TEST_DATABASE));
    }

    #[test]
    fn delete_data() {
        drop_table().unwrap();

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

        prepare
            .write_data(TEST_DATABASE)
            .expect("database does not exist");

        let sample_param = UpdateTask {
            id: 1,
            complete: false,
            delete: true,
        };

        assert_eq!(Ok(()), sample.update_task(sample_param, TEST_DATABASE));
    }

    #[test]
    fn get_all_data() {
        drop_table().unwrap();

        let prepare = TodoData {
            project: String::from("Apple"),
            task: String::from("Test"),
            due_date: String::from("2023-01-01"),
            complete: false,
        };

        prepare
            .write_data(TEST_DATABASE)
            .expect("database does not exist");

        assert_eq!(
            Ok(vec![TodoView {
                id: 1,
                project: String::from("Apple"),
                task: String::from("Test"),
                due_date: String::from("2023-01-01"),
                complete: false,
            }]),
            get_all_tasks(TEST_DATABASE)
        );
    }

    #[test]
    fn get_project_data() {
        drop_table().unwrap();

        let prepare = TodoData {
            project: String::from("Apple"),
            task: String::from("Test"),
            due_date: String::from("2023-01-01"),
            complete: false,
        };

        prepare
            .write_data(TEST_DATABASE)
            .expect("database does not exist");

        assert_eq!(
            Ok(vec![TodoView {
                id: 1,
                project: String::from("Apple"),
                task: String::from("Test"),
                due_date: String::from("2023-01-01"),
                complete: false,
            }]),
            get_tasks("Apple", TEST_DATABASE)
        );
    }
}
