#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub task: SonarBackgroundTask,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SonarBackgroundTask {
    #[serde(rename = "componentKey")]
    component_key: String,
    #[serde(rename = "analysisId")]
    pub analysis_id: String,
    pub status: String,
    #[serde(rename = "executionTimeMs")]
    execution_time_ms: i64,
}
