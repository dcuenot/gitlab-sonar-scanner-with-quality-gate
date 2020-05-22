extern crate serde_json;

pub type MergeRequests = Vec<MergeRequest>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MergeRequest {
    id: i64,
    pub iid: i64,
    pub project_id: i64,
    title: String,
}
