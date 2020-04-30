#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Period {
    index: i64,
    mode: String,
    date: String,
}

#[cfg(test)]
#[allow(non_snake_case)]
pub(in crate::domain) mod tests {
    use super::*;

    pub(in crate::domain) fn default_period() -> Period {
        Period {
            index: 1,
            mode: "previous_version".to_string(),
            date: "2018-11-28T16:40:22+0000".to_string(),
        }
    }
}
