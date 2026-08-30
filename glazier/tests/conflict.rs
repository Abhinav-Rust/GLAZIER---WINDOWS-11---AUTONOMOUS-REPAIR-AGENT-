#[cfg(test)]
mod tests {
    use glazier::conflict::guna::GunaClassifier;
    use glazier::conflict::upaya::UpayaResolver;
    use glazier::execution::karma::{ExecutionOutcome, KarmaExecutor};
    use glazier::memory::working::{ObservationData, TimestampedObservation};
    use glazier::wrl::ast::{Action, GunaType, UpayaItem, UpayaType, VirodhaNode};
    use anyhow::Result;

    struct TestExecutor;
    impl KarmaExecutor for TestExecutor {
        fn execute_action(&self, action: &Action) -> Result<ExecutionOutcome> {
            Ok(ExecutionOutcome {
                success: action.name == "successful_action",
                before_state: "Before".to_string(),
                after_state: "After".to_string(),
                output: "Done".to_string(),
            })
        }
    }

    #[test]
    fn test_guna_classification() {
        let obs_rajas = vec![TimestampedObservation {
            timestamp: chrono::Utc::now(),
            observation: ObservationData::DeviceState {
                name: "mic".to_string(),
                status: "Error".to_string(),
                error_code: Some(43),
            },
        }];
        assert_eq!(GunaClassifier::classify_state(&obs_rajas), GunaType::Rajas);

        let obs_tamas = vec![TimestampedObservation {
            timestamp: chrono::Utc::now(),
            observation: ObservationData::ServiceState {
                name: "Spooler".to_string(),
                status: "stopped".to_string(),
            },
        }];
        assert_eq!(GunaClassifier::classify_state(&obs_tamas), GunaType::Tamas);
    }

    #[test]
    fn test_upaya_resolver_escalation() {
        let virodha = VirodhaNode {
            name: "test_virodha".to_string(),
            improving: "a".to_string(),
            degrades: "b".to_string(),
            guna: GunaType::Rajas,
            upaya: vec![
                UpayaItem {
                    upaya_type: UpayaType::Sama,
                    action: Action {
                        name: "failing_action".to_string(),
                        args: vec![],
                    },
                },
                UpayaItem {
                    upaya_type: UpayaType::Dana,
                    action: Action {
                        name: "successful_action".to_string(),
                        args: vec![],
                    },
                },
            ],
            upeksha_condition: None,
            maya_path: None,
        };

        let executor = TestExecutor;
        let outcome = UpayaResolver::resolve(&virodha, &executor).unwrap();
        assert!(outcome.success);
    }
}
