#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub task: SonarBackgroundTask,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SonarBackgroundTask {
    id: String,
    #[serde(rename = "type")]
    task_type: String,
    #[serde(rename = "componentId")]
    component_id: String,
    #[serde(rename = "componentKey")]
    component_key: String,
    #[serde(rename = "componentName")]
    component_name: String,
    #[serde(rename = "componentQualifier")]
    component_qualifier: String,
    #[serde(rename = "analysisId")]
    pub analysis_id: String,
    pub status: String,
    #[serde(rename = "submittedAt")]
    submitted_at: String,
    #[serde(rename = "submitterLogin")]
    submitter_login: String,
    #[serde(rename = "startedAt")]
    started_at: String,
    #[serde(rename = "executedAt")]
    executed_at: String,
    #[serde(rename = "executionTimeMs")]
    execution_time_ms: i64,
    logs: bool,
    #[serde(rename = "hasScannerContext")]
    has_scanner_context: bool,
    organization: String,
    #[serde(rename = "warningCount")]
    warning_count: i64,
}
