extern crate serde_json;

pub type MergeRequests = Vec<MergeRequest>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MergeRequest {
    pub id: i64,
    pub iid: i64,
    project_id: i64,
    title: String,
}
