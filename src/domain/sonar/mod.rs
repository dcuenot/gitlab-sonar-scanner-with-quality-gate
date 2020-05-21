use project_status::ProjectStatus;

mod condition;
mod period;
mod project_status;
pub mod task;

#[cfg(windows)]
const LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &str = "\n";

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityStatus {
    #[serde(rename = "projectStatus")]
    pub project_status: ProjectStatus,
}

impl QualityStatus {
    pub fn display(self) -> String {
        self.project_status.display()
    }
}
