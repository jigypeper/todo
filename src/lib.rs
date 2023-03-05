use chrono::{Utc};

pub mod database;
pub mod views;
pub mod args;



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date() {
        let today: chrono::DateTime<Utc> = Utc::now();
        assert_eq!(format!("{}", today.format("%Y-%m-%d")), "2023-02-27");
    }
}