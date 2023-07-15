use crate::database::TodoView;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};

use prettytable::Table;

pub fn show_data(data: Vec<TodoView>) -> Table {
    let today: chrono::DateTime<Utc> = Utc::now();
    let mut table = Table::new();
    table.add_row(row!["ID", "PROJECT", "TASK", "DUE DATE", "COMPLETE"]);

    for row in data {
        // use chrono to convert date string into a utc datetime type
        let date = row.due_date.clone();
        let naive_date = match NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
            Ok(date) => date,
            Err(_) => {
                eprintln!(
                    "Date on task '{}' is incorrect, delete and update for correct overdue marking",
                    row.id
                );
                NaiveDate::parse_from_str("2000-01-01", "%Y-%m-%d").unwrap()
            }
        };
        let naive_time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        let naive_date_time = NaiveDateTime::new(naive_date, naive_time);
        let utc_date_time = DateTime::<Utc>::from_utc(naive_date_time, Utc);

        if today > utc_date_time {
            table.add_row(row![
                bFr => row.id,
                row.project,
                row.task,
                row.due_date,
                row.complete
            ]);
        } else {
            table.add_row(row![
                row.id,
                row.project,
                row.task,
                row.due_date,
                row.complete
            ]);
        }
    }

    // return the table
    table
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::TodoView;

    #[test]
    fn test_output() {
        let test_data = vec![TodoView {
            id: 1,
            project: String::from("Apple"),
            task: String::from("Test"),
            due_date: String::from("2023-01-01"),
            complete: false,
        }];

        assert_eq!(
            table!(
                ["ID", "PROJECT", "TASK", "DUE DATE", "COMPLETE"],
                [bFr => 1, "Apple", "Test", "2023-01-01", false]
            ),
            show_data(test_data)
        );
    }
}
