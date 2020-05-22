use crate::domain::sonar::QualityStatus;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    id: i64,
    merge_request_iid: i64,
    body: String,
    system: bool,
}

impl Note {
    pub fn from_quality_status(
        quality_status: QualityStatus,
        project_id: i64,
        merge_request_iid: i64,
    ) -> Self {
        Self {
            id: project_id,
            merge_request_iid,
            body: quality_status.display().replace("\n", "<br />"),
            system: true,
        }
    }
}
