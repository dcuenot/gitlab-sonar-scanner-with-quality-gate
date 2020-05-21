use super::condition::Condition;
use super::period::Period;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectStatus {
    pub status: String,
    pub conditions: Vec<Condition>,
    periods: Vec<Period>,
    #[serde(rename = "ignoredConditions")]
    ignored_conditions: bool,
}

impl ProjectStatus {
    pub(super) fn display(self) -> String {
        let conditions = ProjectStatus::formating_conditions(self.conditions);

        format!(
            "{}{}{}",
            ProjectStatus::format_quality_status(self.status),
            super::LINE_ENDING,
            conditions
        )
    }

    fn format_quality_status(status: String) -> &'static str {
        if status == "OK" {
            "✅ Quality Gate passed"
        } else {
            "⛔️ Quality Gate failed"
        }
    }

    fn formating_conditions(mut conditions: Vec<Condition>) -> String {
        conditions.sort_by(|a, b| a.partial_cmp(b).unwrap());

        conditions
            .into_iter()
            .map(|c| c.display())
            .collect::<Vec<String>>()
            .join(super::LINE_ENDING)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::super::condition::tests::ConditionBuilder;
    use super::*;
    use crate::domain::sonar::condition::tests::assert_contains;
    use crate::domain::sonar::period::tests::default_period;

    #[test]
    fn passed_quality_gate_should_display_green_check() {
        let given = ProjectStatusBuilder::new()
            .with_status("OK")
            .build()
            .display();
        let vec: Vec<&str> = given.split(super::super::LINE_ENDING).collect();

        assert_eq!(vec.len() > 1, true);
        assert_contains(vec[0].into(), "✅ Quality Gate passed");
    }

    #[test]
    fn conditions_should_be_displayed_in_a_sorted_way() {
        let given = ProjectStatusBuilder::new().build().display();
        let vec: Vec<&str> = given.split(super::super::LINE_ENDING).collect();

        assert_eq!(vec.len(), 4);
        assert_contains(vec[0].into(), "⛔️ Quality Gate failed");
        assert_contains(vec[1].into(), "❌");
        assert_contains(vec[2].into(), "✅");
        assert_contains(vec[3].into(), "✅");
    }

    pub(in crate::domain) struct ProjectStatusBuilder<'a> {
        status: &'a str,
        conditions: Vec<Condition>,
        periods: Vec<Period>,
        ignored_conditions: bool,
    }

    impl<'a> ProjectStatusBuilder<'a> {
        pub fn new() -> Self {
            Self {
                status: "ERROR",
                conditions: vec![
                    ConditionBuilder::new().with_status("OK").build(),
                    ConditionBuilder::new().with_status("ERROR").build(),
                    ConditionBuilder::new().with_status("OK").build(),
                ],
                periods: vec![default_period()],
                ignored_conditions: false,
            }
        }

        pub(in crate::domain) fn with_status(&mut self, status: &'a str) -> &mut Self {
            self.status = status;
            self
        }

        pub(in crate::domain) fn build(&self) -> ProjectStatus {
            ProjectStatus {
                status: self.status.into(),
                conditions: self.conditions.clone(),
                periods: self.periods.clone(),
                ignored_conditions: self.ignored_conditions,
            }
        }
    }
}
