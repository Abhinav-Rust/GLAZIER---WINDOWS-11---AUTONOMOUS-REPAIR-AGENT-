use crate::memory::working::ObservationData;
use crate::wrl::ast::GunaType;

pub struct GunaClassifier;

impl GunaClassifier {
    /// Classifies the current system state into one of the three Sāṃkhya Guṇas.
    /// This defines the nature of the issue (or lack thereof).
    pub fn classify_state(
        observations: &[crate::memory::working::TimestampedObservation],
    ) -> GunaType {
        let mut has_conflict = false;
        let mut has_stagnation = false;

        if observations.is_empty() {
            return GunaType::Sattva; // Without observations of issue, assume harmony.
        }

        for obs in observations {
            match &obs.observation {
                ObservationData::DeviceState {
                    status, error_code, ..
                } => {
                    if status.to_lowercase().contains("error") || error_code.is_some() {
                        has_conflict = true; // Rajas (active conflict / error)
                    } else if status.to_lowercase() == "disabled"
                        || status.to_lowercase() == "missing"
                    {
                        has_stagnation = true; // Tamas (inertia / dead)
                    }
                }
                ObservationData::ServiceState { status, .. } => {
                    if status.to_lowercase() == "stopped" || status.to_lowercase() == "suspended" {
                        has_stagnation = true; // Tamas (inertia)
                    } else if status.to_lowercase() == "hung" || status.to_lowercase() == "starting"
                    {
                        has_conflict = true; // Rajas (struggling to start)
                    }
                }
                ObservationData::EventLog { .. } => {
                    has_conflict = true; // Presence of targeted error logs implies Rajas
                }
            }
        }

        // Return based on precedence. Rajas (active conflict) overrides Tamas (stopped).
        if has_conflict {
            GunaType::Rajas
        } else if has_stagnation {
            GunaType::Tamas
        } else {
            GunaType::Sattva // Harmony, correct function
        }
    }
}
