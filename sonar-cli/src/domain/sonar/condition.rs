extern crate inflector;
use inflector::Inflector;

use std::cmp::Ordering;

#[derive(Debug, Serialize, Deserialize, Eq, Clone)]
pub struct Condition {
    status: String,
    #[serde(rename = "metricKey")]
    metric_key: String,
    comparator: String,
    #[serde(rename = "errorThreshold")]
    error_threshold: String,
    #[serde(rename = "actualValue")]
    actual_value: String,
}

impl PartialOrd for Condition {
    fn partial_cmp(&self, other: &Condition) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Condition {
    fn cmp(&self, other: &Condition) -> Ordering {
        self.status.cmp(&other.status)
    }
}

impl PartialEq for Condition {
    fn eq(&self, other: &Condition) -> bool {
        self.status == other.status
    }
}

impl Condition {
    pub(super) fn display(&self) -> String {
        format!(
            "{} {} ({})",
            &self.display_status(),
            &self.display_metric_key(),
            &self.display_assertion()
        )
    }

    fn is_ok(&self) -> bool {
        if &self.status == "OK" {
            return true;
        }
        false
    }

    fn display_status(&self) -> &str {
        if self.is_ok() {
            return "✅";
        }
        "❌"
    }

    fn display_comparator(&self) -> &str {
        match (self.is_ok(), self.comparator.as_str()) {
            (true, "GT") => "<",
            (false, "GT") => "≥",
            (true, "LT") => "≥",
            (false, "LT") => "<",
            _ => &self.comparator,
        }
    }

    fn display_rating(rating: &str) -> &str {
        match rating {
            "1" => "A",
            "2" => "B",
            "3" => "C",
            "4" => "D",
            "5" => "E",
            "6" => "7",
            _ => rating,
        }
    }

    fn display_metric_key(&self) -> String {
        self.metric_key.to_sentence_case()
    }

    fn display_assertion(&self) -> String {
        match &self.metric_key {
            x if (x.contains("coverage") || x.contains("density")) => self.percentage(),
            x if x.ends_with("rating") => self.display_ratings(),
            _ => format!(
                "{} {} {}",
                &self.actual_value,
                &self.display_comparator(),
                &self.error_threshold
            ),
        }
    }

    fn display_ratings(&self) -> String {
        let rating_actual = Condition::display_rating(&self.actual_value).into();
        let rating_threshold = Condition::display_rating(&self.error_threshold);

        match (self.is_ok(), self.actual_value == self.error_threshold) {
            (_, true) => rating_actual,
            (true, _) => format!("{} is better than {}", rating_actual, rating_threshold),
            (false, _) => format!("{} is worse than {}", rating_actual, rating_threshold),
        }
    }

    fn percentage(&self) -> String {
        let actual = &self.actual_value.parse::<f32>().unwrap();
        let error_threshold = &self.error_threshold.parse::<f32>().unwrap();

        format!(
            "{:.0}% {} {:.0}%",
            actual,
            &self.display_comparator(),
            error_threshold
        )
    }
}

// ⛔️✅⚠️❌
#[cfg(test)]
#[allow(non_snake_case)]
pub(super) mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn empty_status_should_display_a_red_cross() {
        let given = ConditionBuilder::new().with_status("").build();
        assert_contains(given.display(), "❌")
    }

    #[test]
    fn a_KO_status_should_display_a_red_cross() {
        let given = ConditionBuilder::new().with_status("ERROR").build();
        assert_contains(given.display(), "❌")
    }

    #[test]
    fn a_OK_status_should_display_a_green_check() {
        let given = ConditionBuilder::new().with_status("OK").build();
        assert_contains(given.display(), "✅")
    }

    #[test]
    fn metric_key_should_be_display_starting_with_uppercase_and_without_underscore() {
        let given = ConditionBuilder::new()
            .with_metric_key("reliability_rating")
            .build();
        assert_contains(given.display(), "Reliability rating")
    }

    proptest! {
        #[test]
        fn for_ratings_if_actual_equals_expected_display_should_be_simplified(y in -100..100i32) {
            let s = &y.to_string();
            assert_contains(
                ConditionBuilder::new()
                    .with_metric_key("rating")
                    .with_comparaison(s, "GT", s)
                    .build()
                    .display(),
                &format!("({})", Condition::display_rating(s)),
            );
        }
    }

    #[test]
    fn reliability_rating_with_1_LT_2_should_display_A_better_B() {
        let given = ConditionBuilder::new()
            .with_metric_key("reliability_rating")
            .with_comparaison("1", "LT", "2")
            .build();
        assert_contains(
            given.display(),
            "✅ Reliability rating (A is better than B)",
        )
    }

    #[test]
    fn new_reliability_rating_with_4_GT_1_should_display_D_worse_B() {
        let given = ConditionBuilder::new()
            .with_metric_key("new_reliability_rating")
            .with_status("ERROR")
            .with_comparaison("4", "GT", "2")
            .build();
        assert_contains(
            given.display(),
            "❌ New reliability rating (D is worse than B)",
        )
    }

    #[test]
    fn random_should_display_with_fallback_display() {
        let given = ConditionBuilder::new()
            .with_metric_key("random")
            .with_status("OK")
            .with_comparaison("4", "GT", "298")
            .build();
        assert_contains(given.display(), "✅ Random (4 < 298)")
    }

    #[test]
    fn new_coverage_with_0_LT_80_should_display_0percent_lt_80percent() {
        let given = ConditionBuilder::new()
            .with_metric_key("new_coverage")
            .with_status("ERROR")
            .with_comparaison("0.0", "LT", "80")
            .build();
        assert_contains(given.display(), "❌ New coverage (0% < 80%)")
    }

    #[test]
    fn new_duplicated_lines_density_with_6_33162_GT_3_should_display_6percent_gte_3percent() {
        let given = ConditionBuilder::new()
            .with_metric_key("new_duplicated_lines_density")
            .with_status("ERROR")
            .with_comparaison("0.0", "LT", "80")
            .build();
        assert_contains(
            given.display(),
            "❌ New duplicated lines density (0% < 80%)",
        )
    }

    #[test]
    fn coverage_with_85_8_LT_80_should_display_86percent_gte_80percent() {
        let given = ConditionBuilder::new()
            .with_metric_key("coverage")
            .with_status("OK")
            .with_comparaison("85.8", "LT", "80")
            .build();
        assert_contains(given.display(), "✅ Coverage (86% ≥ 80%)")
    }

    #[test]
    fn duplicated_lines_density_with_0_GT_3_should_display_6percent_gte_3percent() {
        let given = ConditionBuilder::new()
            .with_metric_key("duplicated_lines_density")
            .with_status("OK")
            .with_comparaison("0.0", "GT", "3")
            .build();
        assert_contains(given.display(), "✅ Duplicated lines density (0% < 3%)")
    }

    pub(in crate::domain) struct ConditionBuilder<'a> {
        status: &'a str,
        metric_key: &'a str,
        comparator: &'a str,
        error_threshold: &'a str,
        actual_value: &'a str,
    }

    impl<'a> ConditionBuilder<'a> {
        pub fn new() -> Self {
            Self {
                status: "OK",
                metric_key: "reliability_rating",
                comparator: "GT",
                error_threshold: "1",
                actual_value: "1",
            }
        }

        pub(in crate::domain) fn with_status(&mut self, status: &'a str) -> &mut Self {
            self.status = status;
            self
        }

        pub(in crate::domain) fn with_metric_key(&mut self, metric_key: &'a str) -> &mut Self {
            self.metric_key = metric_key;
            self
        }

        pub(in crate::domain) fn with_comparaison(
            &mut self,
            actual_value: &'a str,
            comparator: &'a str,
            error_threshold: &'a str,
        ) -> &mut Self {
            self.actual_value = actual_value;
            self.comparator = comparator;
            self.error_threshold = error_threshold;
            self
        }

        pub(in crate::domain) fn build(&self) -> Condition {
            Condition {
                status: self.status.into(),
                metric_key: self.metric_key.to_string(),
                comparator: self.comparator.to_string(),
                error_threshold: self.error_threshold.to_string(),
                actual_value: self.actual_value.to_string(),
            }
        }
    }

    pub(in crate::domain) fn assert_contains(result: String, expected: &str) {
        if !result.contains(expected) {
            panic!(
                r#"assertion failed: `result.contains(expected)`
 result: `{:?}`,
 expected: `{:?}`"#,
                &*result, &*expected
            )
        }
    }
}
