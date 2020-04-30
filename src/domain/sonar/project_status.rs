use super::condition::Condition;
use super::period::Period;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectStatus {
    pub status: String,
    pub conditions: Vec<Condition>,
    periods: Vec<Period>,
    #[serde(rename = "ignoredConditions")]
    ignored_conditions: bool,
}
