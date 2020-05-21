use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(PartialEq, Debug)]
pub(crate) struct SonarAnalysisParams {
    project_key: String,
    pub server_url: String,
    server_version: String,
    dashboard_url: String,
    pub ce_task_id: String,
    ce_task_url: String,
}

impl SonarAnalysisParams {
    #[allow(dead_code)]
    fn new(
        project_key: &str,
        server_url: &str,
        server_version: &str,
        dashboard_url: &str,
        ce_task_id: &str,
        ce_task_url: &str,
    ) -> Self {
        Self {
            project_key: project_key.into(),
            server_url: server_url.into(),
            server_version: server_version.into(),
            dashboard_url: dashboard_url.into(),
            ce_task_id: ce_task_id.into(),
            ce_task_url: ce_task_url.into(),
        }
    }

    pub fn from_report_task(report_task_path: PathBuf) -> Self {
        let props = Self::read_file(&report_task_path);

        Self {
            project_key: props.get("projectKey").unwrap().into(),
            server_url: props.get("serverUrl").unwrap().into(),
            server_version: props.get("serverVersion").unwrap().into(),
            dashboard_url: props.get("dashboardUrl").unwrap().into(),
            ce_task_id: props.get("ceTaskId").unwrap().into(),
            ce_task_url: props.get("ceTaskUrl").unwrap().into(),
        }
    }

    ///
    /// Read a file with this kind of content:
    /// project_key=my-awesome-project
    /// serverUrl=https://sonar.com/sonar
    ///
    /// And return an HashMap<String, String>
    /// {
    ///     "projectKey": "my-awesome-project"
    ///     "serverUrl": "https://sonar.com/sonar"
    /// }
    ///
    fn read_file<P: AsRef<Path>>(file_path: P) -> HashMap<String, String> {
        File::open(file_path)
            .map_err(|e| eprintln!("Impossible to open file : {}", e))
            .and_then(|mut file| {
                let mut buffer = String::new();
                file.read_to_string(&mut buffer)
                    .map_err(|err| eprintln!("{}", err))
                    .map(move |_| buffer)
            })
            .map(|line| {
                line.lines()
                    .map(|l| {
                        let s: String = l.into();
                        let pos = s.clone().find('=').expect("No char '=' found");
                        (s[..pos].to_string(), s[(pos + 1)..].to_string())
                    })
                    .collect::<HashMap<String, String>>()
            })
            .expect("Something wrong happend")
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn check_stub() -> () {
        let from_file = SonarAnalysisParams::from_report_task(
            "./src/tests/test-report-tasks.txt".parse().unwrap(),
        );
        let stub = given_a_report_task();

        assert_eq!(from_file, stub)
    }

    fn given_a_report_task() -> SonarAnalysisParams {
        SonarAnalysisParams::new(
            "my-awesome-project",
            "https://sonar.com/sonar",
            "7.9.1.27448",
            "https://sonar.com/sonar/dashboard?id=my-awesome-project",
            "AXIxuGnQQLkOZHj6WkoG",
            "https://sonar.com/sonar/api/ce/task?id=AXIxuGnQQLkOZHj6WkoG",
        )
    }
}
