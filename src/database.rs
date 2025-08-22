use crate::args::UpdateTask;
use rusqlite::{named_params, params, Connection, Result};

pub struct TodoData {
    pub project: String,
    pub task: String,
    pub due_date: String,
    pub complete: bool,
}

impl TodoData {
    /// Writes task data to the database
    ///
    /// # Errors
    ///
    /// Returns an error if database operations fail
    ///
    /// # Panics
    ///
    /// This function will panic if the database connection cannot be established
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
            "INSERT INTO data (project, task, due_date, complete)
            VALUES (:project, :task, :due_date, :complete)",
            named_params! {
                ":project": self.project,
                ":task": self.task,
                ":due_date": self.due_date,
                ":complete": i32::from(self.complete),
            },
        )?;

        tx.commit()?;

        Ok(())
    }

    /// Updates a task in the database
    ///
    /// # Errors
    ///
    /// Returns an error if database operations fail
    ///
    /// # Panics
    ///
    /// This function will panic if the database connection cannot be established
    pub fn update_task(self, update_task: UpdateTask, db_file: &str) -> Result<()> {
        if update_task.complete && update_task.delete {
            println!("Cannot delete and update a task");
        } else if update_task.complete {
            let mut conn = Connection::open(db_file).unwrap();

            let tx = conn.transaction()?;
            tx.execute(
                "UPDATE data
                SET complete = :complete
                WHERE id = :id",
                named_params! {
                    ":id": update_task.id,
                    ":complete": i32::from(self.complete),
                },
            )?;

            tx.commit()?;
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
        }
        Ok(())
    }

    /// Archives the task
    ///
    /// # Errors
    ///
    /// Returns an error if archiving fails
    pub fn archive(self) -> Result<()> {
        // is this a requirement?
        // perhaps needs to push to online db?
        // maybe needs an api as a microservice
        todo!();
    }

    /// Sets a reminder for the task
    ///
    /// # Errors
    ///
    /// Returns an error if setting reminder fails
    pub fn set_reminder(self) -> Result<()> {
        todo!();
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TodoView {
    pub id: u64,
    pub project: String,
    pub task: String,
    pub due_date: String,
    pub complete: bool,
}

/// Gets tasks for a specific project
///
/// # Errors
///
/// Returns an error if database operations fail
///
/// # Panics
///
/// This function will panic if the database connection cannot be established
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
            complete: matches!(row.get(4).unwrap(), 1),
        })
    })?;

    let mut result = Vec::new();

    for task in tasks_iter {
        result.push(task?);
    }

    Ok(result)
}

/// Gets all tasks from the database
///
/// # Errors
///
/// Returns an error if database operations fail
///
/// # Panics
///
/// This function will panic if the database connection cannot be established
pub fn get_all_tasks(db_file: &str) -> Result<Vec<TodoView>> {
    let conn = Connection::open(db_file).unwrap();

    let mut stmt = conn.prepare("SELECT * FROM data;")?;

    let tasks_iter = stmt.query_map([], |row| {
        Ok(TodoView {
            id: row.get(0)?,
            project: row.get(1)?,
            task: row.get(2)?,
            due_date: row.get(3)?,
            complete: matches!(row.get(4).unwrap(), 1),
        })
    })?;

    let mut result = Vec::new();

    for task in tasks_iter {
        result.push(task?);
    }

    Ok(result)
}

/// Counts pending tasks in the database
///
/// # Errors
///
/// Returns an error if database operations fail
///
/// # Panics
///
/// This function will panic if the database connection cannot be established
pub fn count_pending(db_file: &str) -> Result<u32> {
    let conn = Connection::open(db_file).unwrap();

    let count: u32 = conn.query_row(
        "SELECT COUNT(*) FROM data WHERE complete = 0",
        params![],
        |row| row.get(0),
    )?;

    Ok(count)
}

/// Counts overdue tasks in the database
///
/// # Errors
///
/// Returns an error if database operations fail
///
/// # Panics
///
/// This function will panic if the database connection cannot be established
pub fn count_overdue(db_file: &str) -> Result<u32> {
    let conn = Connection::open(db_file).unwrap();

    let count: u32 = conn.query_row(
        "SELECT COUNT(*) FROM data WHERE complete = 0 AND due_date < CURRENT_DATE",
        params![],
        |row| row.get(0),
    )?;

    Ok(count)
}

/// Archives a task by moving it from the main table to the archive table
///
/// # Errors
///
/// Returns an error if database operations fail or if the task doesn't exist
///
/// # Panics
///
/// This function will panic if the database connection cannot be established
pub fn archive_task(task_id: u64, db_file: &str) -> Result<()> {
    let mut conn = Connection::open(db_file).unwrap();

    // Create archive table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS archived_data (
            id INTEGER PRIMARY KEY NOT NULL,
            project VARCHAR(50) NOT NULL,
            task VARCHAR(100) NOT NULL,
            due_date DATE,
            complete BOOLEAN NOT NULL CHECK (complete IN (0, 1)),
            archived_date DATETIME DEFAULT CURRENT_TIMESTAMP
        );",
        (),
    )?;

    let tx = conn.transaction()?;

    // First, get the task data
    let task_data: Result<(String, String, String, bool), rusqlite::Error> = {
        let mut stmt = tx.prepare("SELECT project, task, due_date, complete FROM data WHERE id = ?")?;
        stmt.query_row(params![task_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                matches!(row.get::<_, i32>(3)?, 1),
            ))
        })
    };

    match task_data {
        Ok((project, task, due_date, complete)) => {
            // Insert into archive table
            tx.execute(
                "INSERT INTO archived_data (project, task, due_date, complete)
                VALUES (?1, ?2, ?3, ?4)",
                params![project, task, due_date, i32::from(complete)],
            )?;

            // Delete from main table
            tx.execute("DELETE FROM data WHERE id = ?1", params![task_id])?;

            tx.commit()?;
            Ok(())
        }
        Err(_) => Err(rusqlite::Error::QueryReturnedNoRows),
    }
}

/// Gets all archived tasks from the database
///
/// # Errors
///
/// Returns an error if database operations fail
///
/// # Panics
///
/// This function will panic if the database connection cannot be established
pub fn get_all_archived_tasks(db_file: &str) -> Result<Vec<TodoView>> {
    let conn = Connection::open(db_file).unwrap();

    // Create archive table if it doesn't exist (in case this is called before archiving anything)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS archived_data (
            id INTEGER PRIMARY KEY NOT NULL,
            project VARCHAR(50) NOT NULL,
            task VARCHAR(100) NOT NULL,
            due_date DATE,
            complete BOOLEAN NOT NULL CHECK (complete IN (0, 1)),
            archived_date DATETIME DEFAULT CURRENT_TIMESTAMP
        );",
        (),
    )?;

    let mut stmt = conn.prepare("SELECT id, project, task, due_date, complete FROM archived_data ORDER BY id;")?;

    let tasks_iter = stmt.query_map([], |row| {
        Ok(TodoView {
            id: row.get(0)?,
            project: row.get(1)?,
            task: row.get(2)?,
            due_date: row.get(3)?,
            complete: matches!(row.get(4).unwrap(), 1),
        })
    })?;

    let mut result = Vec::new();

    for task in tasks_iter {
        result.push(task?);
    }

    Ok(result)
}

/// Gets archived tasks for a specific project
///
/// # Errors
///
/// Returns an error if database operations fail
///
/// # Panics
///
/// This function will panic if the database connection cannot be established
pub fn get_archived_tasks(project_name: &str, db_file: &str) -> Result<Vec<TodoView>> {
    let conn = Connection::open(db_file).unwrap();

    // Create archive table if it doesn't exist (in case this is called before archiving anything)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS archived_data (
            id INTEGER PRIMARY KEY NOT NULL,
            project VARCHAR(50) NOT NULL,
            task VARCHAR(100) NOT NULL,
            due_date DATE,
            complete BOOLEAN NOT NULL CHECK (complete IN (0, 1)),
            archived_date DATETIME DEFAULT CURRENT_TIMESTAMP
        );",
        (),
    )?;

    let mut stmt = conn.prepare(
        "SELECT id, project, task, due_date, complete FROM archived_data
         WHERE project = :project_name ORDER BY id;",
    )?;

    let tasks_iter = stmt.query_map(&[(":project_name", &project_name)], |row| {
        Ok(TodoView {
            id: row.get(0)?,
            project: row.get(1)?,
            task: row.get(2)?,
            due_date: row.get(3)?,
            complete: matches!(row.get(4).unwrap(), 1),
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
        conn.execute("DROP TABLE IF EXISTS archived_data;", ())?;

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

    #[test]
    fn get_pending_count() {
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

        assert_eq!(Ok(1), count_pending(TEST_DATABASE));
    }

    #[test]
    fn get_overdue_count() {
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

        assert_eq!(Ok(1), count_overdue(TEST_DATABASE));
    }

    #[test]
    fn test_archive_task() {
        drop_table().unwrap();

        // Create a task first
        let task = TodoData {
            project: String::from("Work"),
            task: String::from("Complete presentation"),
            due_date: String::from("2023-01-15"),
            complete: false,
        };

        task.write_data(TEST_DATABASE).expect("Failed to create task");

        // Archive the task
        assert_eq!(Ok(()), archive_task(1, TEST_DATABASE));

        // Verify task is removed from main table
        let main_tasks = get_all_tasks(TEST_DATABASE).unwrap();
        assert_eq!(main_tasks.len(), 0);

        // Verify task is in archive table
        let archived_tasks = get_all_archived_tasks(TEST_DATABASE).unwrap();
        assert_eq!(archived_tasks.len(), 1);
        assert_eq!(archived_tasks[0].project, "Work");
        assert_eq!(archived_tasks[0].task, "Complete presentation");
        
        drop_table().unwrap();
    }

    #[test]
    fn archive_nonexistent_task() {
        drop_table().unwrap();

        // Try to archive a task that doesn't exist
        let result = archive_task(999, TEST_DATABASE);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_all_archived_tasks() {
        drop_table().unwrap();

        // Create and archive multiple tasks
        let task1 = TodoData {
            project: String::from("Work"),
            task: String::from("Task 1"),
            due_date: String::from("2023-01-01"),
            complete: true,
        };

        let task2 = TodoData {
            project: String::from("Personal"),
            task: String::from("Task 2"),
            due_date: String::from("2023-01-02"),
            complete: false,
        };

        task1.write_data(TEST_DATABASE).expect("Failed to create task1");
        task2.write_data(TEST_DATABASE).expect("Failed to create task2");

        // Archive both tasks
        archive_task(1, TEST_DATABASE).expect("Failed to archive task1");
        archive_task(2, TEST_DATABASE).expect("Failed to archive task2");

        // Get all archived tasks
        let archived = get_all_archived_tasks(TEST_DATABASE).unwrap();
        assert_eq!(archived.len(), 2);

        // Verify main table is empty
        let main_tasks = get_all_tasks(TEST_DATABASE).unwrap();
        assert_eq!(main_tasks.len(), 0);
        
        drop_table().unwrap();
    }

    #[test]
    fn get_archived_tasks_by_project() {
        drop_table().unwrap();

        // Create tasks in different projects
        let work_task = TodoData {
            project: String::from("Work"),
            task: String::from("Work task"),
            due_date: String::from("2023-01-01"),
            complete: false,
        };

        let personal_task = TodoData {
            project: String::from("Personal"),
            task: String::from("Personal task"),
            due_date: String::from("2023-01-02"),
            complete: false,
        };

        work_task.write_data(TEST_DATABASE).expect("Failed to create work task");
        personal_task.write_data(TEST_DATABASE).expect("Failed to create personal task");

        // Archive both tasks
        archive_task(1, TEST_DATABASE).expect("Failed to archive work task");
        archive_task(2, TEST_DATABASE).expect("Failed to archive personal task");

        // Get archived tasks for Work project only
        let work_archived = get_archived_tasks("Work", TEST_DATABASE).unwrap();
        assert_eq!(work_archived.len(), 1);
        assert_eq!(work_archived[0].project, "Work");
        assert_eq!(work_archived[0].task, "Work task");

        // Get archived tasks for Personal project only
        let personal_archived = get_archived_tasks("Personal", TEST_DATABASE).unwrap();
        assert_eq!(personal_archived.len(), 1);
        assert_eq!(personal_archived[0].project, "Personal");
        assert_eq!(personal_archived[0].task, "Personal task");
        
        drop_table().unwrap();
    }

    #[test]
    fn archive_task_transaction_integrity() {
        drop_table().unwrap();

        // Create a task
        let task = TodoData {
            project: String::from("Test"),
            task: String::from("Transaction test"),
            due_date: String::from("2023-01-01"),
            complete: false,
        };

        task.write_data(TEST_DATABASE).expect("Failed to create task");

        // Archive the task
        archive_task(1, TEST_DATABASE).expect("Failed to archive task");

        // Verify exactly one task in archive, zero in main
        let main_count = get_all_tasks(TEST_DATABASE).unwrap().len();
        let archive_count = get_all_archived_tasks(TEST_DATABASE).unwrap().len();
        
        assert_eq!(main_count, 0, "Task should be removed from main table");
        assert_eq!(archive_count, 1, "Task should exist in archive table");
    }
}
