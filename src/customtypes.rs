/// This module contains custom-defined data types.

/// Custom data type to collect the upcoming releases.
#[derive(Debug)]
pub struct UpcomingRelease {
    pub author: String,
    pub title: String,
    pub date: chrono::DateTime<chrono::Utc>,
}

impl UpcomingRelease {
    pub fn create(author: String, title: String, date: chrono::DateTime<chrono::Utc>) -> Self {
        UpcomingRelease {
            author,
            title,
            date,
        }
    }
}
