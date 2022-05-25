use crate::{Crd, TaskState};
use k8s_openapi::api::core::v1::PodStatus;
use kube::{core::object::HasStatus, ResourceExt};
use serde::Serialize;
use tabled::object::Segment;
use tabled::{Alignment, MaxWidth, MinWidth, Modify, Style, Table, Tabled};

/// `StatusSnapshot` represents the status of a set of testsys objects (including the controller).
/// `StatusSnapshot::to_string()` is used to create a table representation of the status.
/// `StatusSnapshot` can also be used with `json::to_string()` to create a json representation of
/// the testsys objects.
#[derive(Debug, Serialize)]
pub struct StatusSnapshot {
    finished: bool,
    passed: bool,
    failed_tests: Vec<String>,
    controller_status: Option<PodStatus>,
    crds: Vec<Crd>,
}

impl StatusSnapshot {
    pub(super) fn new(controller_status: Option<PodStatus>, crds: Vec<Crd>) -> Self {
        let mut passed = true;
        let mut finished = true;
        let mut failed_tests = Vec::new();
        for crd in &crds {
            match crd {
                Crd::Test(test) => match test.agent_status().task_state {
                    TaskState::Unknown | TaskState::Running => {
                        passed = false;
                        finished = false
                    }
                    TaskState::Error => {
                        passed = false;
                        failed_tests.push(test.name());
                    }
                    _ => continue,
                },
                Crd::Resource(resource) => {
                    match resource.creation_task_state() {
                        TaskState::Unknown | TaskState::Running => {
                            passed = false;
                            finished = false
                        }
                        TaskState::Error => passed = false,
                        _ => continue,
                    };
                    match resource.destruction_task_state() {
                        TaskState::Unknown | TaskState::Running => {
                            // Indicate that some pods still may be running.
                            finished = false
                        }
                        _ => continue,
                    }
                }
            }
        }
        Self {
            passed,
            finished,
            failed_tests,
            controller_status,
            crds,
        }
    }
}

impl std::fmt::Display for StatusSnapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let table: Table = self.into();
        if let Some(width) = f.width() {
            // If we received a width, we use it
            write!(
                f,
                "{}",
                table
                    .with(MaxWidth::truncating(width))
                    .with(MinWidth::new(width))
            )
        } else {
            // Otherwise we do nothing special
            write!(f, "{}", table)
        }
    }
}

impl From<&StatusSnapshot> for Table {
    fn from(status: &StatusSnapshot) -> Self {
        let mut results = Vec::new();
        if let Some(controller_status) = &status.controller_status {
            results.push(ResultRow {
                name: "controller".to_string(),
                object_type: "Controller".to_string(),
                state: controller_status
                    .phase
                    .clone()
                    .unwrap_or_else(|| "".to_string()),
                passed: None,
                skipped: None,
                failed: None,
            });
        }
        for crd in &status.crds {
            results.extend::<Vec<ResultRow>>(crd.into());
        }
        results.sort_by(|a, b| a.name.cmp(&b.name));

        Table::new(results)
            .with(Style::blank())
            .with(Modify::new(Segment::all()).with(Alignment::left()))
    }
}

#[derive(Tabled, Default, Clone, Serialize)]
struct ResultRow {
    #[tabled(rename = "NAME")]
    name: String,
    #[tabled(rename = "TYPE")]
    object_type: String,
    #[tabled(rename = "STATE")]
    state: String,
    #[tabled(rename = "PASSED")]
    #[tabled(display_with = "display_option")]
    passed: Option<u64>,
    #[tabled(rename = "SKIPPED")]
    #[tabled(display_with = "display_option")]
    skipped: Option<u64>,
    #[tabled(rename = "FAILED")]
    #[tabled(display_with = "display_option")]
    failed: Option<u64>,
}

fn display_option(o: &Option<u64>) -> String {
    match o {
        Some(count) => format!("{}", count),
        None => "".to_string(),
    }
}

impl From<&Crd> for Vec<ResultRow> {
    fn from(crd: &Crd) -> Self {
        let mut results = Vec::new();
        match crd {
            Crd::Test(test) => {
                let name = test.metadata.name.clone().unwrap_or_else(|| "".to_string());
                let state = test.test_user_state().to_string();
                let test_results = &test.agent_status().results;
                if test_results.is_empty() {
                    results.push(ResultRow {
                        name,
                        object_type: "Test".to_string(),
                        state,
                        passed: None,
                        skipped: None,
                        failed: None,
                    })
                } else {
                    for (test_count, result) in test_results.iter().enumerate() {
                        let retry_name = if test_count == 0 {
                            name.clone()
                        } else {
                            format!("{}-retry-{}", name, test_count)
                        };
                        results.push(ResultRow {
                            name: retry_name,
                            object_type: "Test".to_string(),
                            state: state.clone(),
                            passed: Some(result.num_passed),
                            skipped: Some(result.num_skipped),
                            failed: Some(result.num_failed),
                        });
                    }
                }
            }
            Crd::Resource(resource) => {
                let name = resource.name();
                let mut create_state = TaskState::Unknown;
                let mut delete_state = TaskState::Unknown;
                if let Some(status) = resource.status() {
                    create_state = status.creation.task_state;
                    delete_state = status.destruction.task_state;
                }
                let state = match delete_state {
                    TaskState::Unknown => create_state,
                    _ => delete_state,
                };

                results.push(ResultRow {
                    name,
                    object_type: "Resource".to_string(),
                    state: state.to_string(),
                    passed: None,
                    skipped: None,
                    failed: None,
                });
            }
        };
        results
    }
}
