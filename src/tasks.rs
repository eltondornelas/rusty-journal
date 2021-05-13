use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;

/*
 * We won't add a status or is_complete field because we'll represent the to-do list as a vector
 * of tasks (Vec<Task>). So when a task is complete, we can simply remove it from the vector.
 **************************************************************************************************/

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}