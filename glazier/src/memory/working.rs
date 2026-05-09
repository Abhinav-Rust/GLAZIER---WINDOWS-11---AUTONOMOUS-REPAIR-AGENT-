use crate::wrl::ast::{Component, GunaType};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct WorkingMemory {
    pub current_target: Option<Component>,
    pub observations: Vec<TimestampedObservation>,
    pub active_hypotheses: Vec<Hypothesis>,
    pub ruled_out: Vec<Hypothesis>,
    pub current_guna: Option<GunaType>,
    pub fix_history: Vec<AttemptedFix>,
}

#[derive(Debug, Clone)]
pub struct TimestampedObservation {
    pub timestamp: DateTime<Utc>,
    pub observation: ObservationData,
}

#[derive(Debug, Clone)]
pub enum ObservationData {
    DeviceState { name: String, status: String, error_code: Option<i64> },
    ServiceState { name: String, status: String },
    EventLog { source: String, event_id: u32 },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Hypothesis {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct AttemptedFix {
    pub action_name: String,
    pub outcome: FixOutcome,
}

#[derive(Debug, Clone)]
pub enum FixOutcome {
    Success,
    Failure,
    Partial,
}

impl WorkingMemory {
    pub fn new() -> Self {
        Self {
            current_target: None,
            observations: Vec::new(),
            active_hypotheses: Vec::new(),
            ruled_out: Vec::new(),
            current_guna: None,
            fix_history: Vec::new(),
        }
    }
}
