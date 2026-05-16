use crate::wrl::ast::{AnumanaNode, Predicate};
use crate::memory::working::{WorkingMemory, ObservationData};
use crate::wrl::validator::{HetvabhasaError, HetvabhasaType};
use chrono::Utc;

pub struct HetvabhasaGuard;

impl HetvabhasaGuard {
    pub fn check(anumana: &AnumanaNode, wm: &WorkingMemory) -> Result<(), HetvabhasaError> {
        match &anumana.hetu {
            Predicate::State(state) => {
                let mut found = false;
                let mut stale = false;

                for obs in &wm.observations {
                    // Check staleness (e.g., older than 5 minutes)
                    let now = Utc::now();
                    if now.signed_duration_since(obs.timestamp).num_minutes() > 5 {
                        stale = true;
                    }

                    match &obs.observation {
                        ObservationData::DeviceState { name: _, error_code, .. } => {
                            if state.property.name == "error_code" && state.property.args.len() > 0 {
                                // extremely simplified match
                                if let Some(code) = error_code {
                                    if state.property.args[0] == crate::wrl::ast::PropertyArg::Integer(*code) {
                                        found = true;
                                        break;
                                    }
                                }
                            }
                        },
                        _ => {} // Handle other cases similarly
                    }
                }

                if !found {
                    return Err(HetvabhasaError {
                        anumana_name: anumana.name.clone(),
                        fallacy_type: HetvabhasaType::Asiddha,
                        explanation: "Hetu condition not found in current observations.".to_string(),
                    });
                }

                if stale {
                     return Err(HetvabhasaError {
                        anumana_name: anumana.name.clone(),
                        fallacy_type: HetvabhasaType::Katatita,
                        explanation: "Observation is stale.".to_string(),
                    });
                }
            },
            _ => {}
        }

        Ok(())
    }
}
