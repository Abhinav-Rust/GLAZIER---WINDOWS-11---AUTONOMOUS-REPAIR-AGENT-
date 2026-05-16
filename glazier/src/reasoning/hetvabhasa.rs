use crate::wrl::ast::{AnumanaNode, Predicate};
use crate::memory::working::{WorkingMemory, ObservationData};
use crate::wrl::validator::{HetvabhasaError, HetvabhasaType};
use chrono::Utc;

pub struct HetvabhasaGuard;

impl HetvabhasaGuard {
    pub fn check(anumana: &AnumanaNode, wm: &WorkingMemory) -> Result<(), HetvabhasaError> {
        // 1. Prakaranasama Check (circular reasoning):
        // If the nigamana action name exactly matches the hetu property name (rudimentary check).
        if let Predicate::State(state) = &anumana.hetu {
            if anumana.nigamana.name == state.property.name {
                return Err(HetvabhasaError {
                    anumana_name: anumana.name.clone(),
                    fallacy_type: HetvabhasaType::Prakaranasama,
                    explanation: format!("Nigamana action '{}' references its own hetu property.", anumana.nigamana.name),
                });
            }
        }

        // 2. Viruddha Check (contradiction):
        // Highly simplified: Does the action attempt to start something that is already explicitly marked as running?
        // Or uninstall something that is already uninstalled?
        // This would require deep domain knowledge, but we simulate a basic check here.
        if let Predicate::State(state) = &anumana.hetu {
            if anumana.nigamana.name == "start_service" {
                if let Some(expected) = &state.expected_state {
                    if expected == "running" {
                        return Err(HetvabhasaError {
                            anumana_name: anumana.name.clone(),
                            fallacy_type: HetvabhasaType::Viruddha,
                            explanation: "Hetu states service is running, but Nigamana attempts to start it. Contradiction.".to_string(),
                        });
                    }
                }
            }
        }

        // 3. Asiddha & 4. Katatita Check:
        match &anumana.hetu {
            Predicate::State(state) => {
                let mut found = false;
                let mut stale = false;

                for obs in &wm.observations {
                    // Check staleness (Kālātīta)
                    // The architect specified Kālātīta guards against stale observations. Let's use 5 minutes.
                    let now = Utc::now();
                    if now.signed_duration_since(obs.timestamp).num_minutes() > 5 {
                        stale = true;
                    }

                    match &obs.observation {
                        ObservationData::DeviceState { error_code, .. } => {
                            if state.property.name == "error_code" && !state.property.args.is_empty() {
                                if let Some(code) = error_code {
                                    if state.property.args[0] == crate::wrl::ast::PropertyArg::Integer(*code) {
                                        found = true;
                                        break;
                                    }
                                }
                            }
                        },
                        ObservationData::ServiceState { name, status } => {
                            if state.property.name == "service_state" && !state.property.args.is_empty() {
                                if state.property.args[0] == crate::wrl::ast::PropertyArg::Ident(name.clone()) {
                                    if let Some(expected) = &state.expected_state {
                                        if status == expected {
                                            found = true;
                                            break;
                                        }
                                    }
                                }
                            }
                        },
                        _ => {}
                    }
                }

                // Asiddha: The reason (Hetu) is unproven because it wasn't observed.
                if !found {
                    return Err(HetvabhasaError {
                        anumana_name: anumana.name.clone(),
                        fallacy_type: HetvabhasaType::Asiddha,
                        explanation: "Hetu condition not found in current observations (Pratyaksha). Cannot reason from unproven assumptions.".to_string(),
                    });
                }

                // Katatita: The observation exists but is too old to be reliable.
                if stale {
                     return Err(HetvabhasaError {
                        anumana_name: anumana.name.clone(),
                        fallacy_type: HetvabhasaType::Katatita,
                        explanation: "Observation is stale (exceeds time threshold). Require fresh Pratyaksha.".to_string(),
                    });
                }
            },
            _ => {}
        }

        // 5. Anaikantika (Ambiguity/Wandering Reason):
        // This would require querying the VyaptiStore to see if the given Hetu matches *multiple* rules
        // leading to different conclusions. We simulate the interface here.
        // If we found ambiguous rules, we would return:
        // return Err(HetvabhasaError {
        //     anumana_name: anumana.name.clone(),
        //     fallacy_type: HetvabhasaType::Anaikantika,
        //     explanation: "Hetu matches multiple Vyaptis leading to ambiguous conclusions. Require disambiguation.".to_string(),
        // });

        Ok(())
    }
}
