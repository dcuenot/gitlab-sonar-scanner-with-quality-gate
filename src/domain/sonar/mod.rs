use project_status::ProjectStatus;

mod condition;
mod period;
mod project_status;

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityStatus {
    #[serde(rename = "projectStatus")]
    pub project_status: ProjectStatus,
}
