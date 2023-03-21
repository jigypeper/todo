// use chrono::Utc;

#[macro_use]
extern crate prettytable;

pub mod args;
pub mod data_handler;
pub mod database;
pub mod views;

// pub const DBFILE: &str = "todo.db";

// pub const TODAY: chrono::DateTime<Utc> = Utc::now();
// pub static DEFAULT_DATE: chrono::DateTime<Utc> = TODAY + Duration::days(7);

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn date() {
//         let today: chrono::DateTime<Utc> = Utc::now();
//         assert_eq!(format!("{}", today.format("%Y-%m-%d")), "2023-02-27");
//     }
// }
