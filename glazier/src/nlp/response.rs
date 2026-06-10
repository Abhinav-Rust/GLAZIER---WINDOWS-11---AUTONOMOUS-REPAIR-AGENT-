use std::collections::HashMap;

pub enum AgentState {
    DiagnosisStarted,
    HypothesisFormed,
    ActionTaken,
    FixVerified,
    HetvabhasaTriggered,
    UnknownState,
    UpekshaApplied,
}

pub struct ResponseGenerator {
    templates: HashMap<String, String>,
}

impl ResponseGenerator {
    pub fn new() -> Self {
        let mut templates = HashMap::new();
        templates.insert(
            "diagnosis_started".to_string(),
            "Observing {target}. Checking {properties}.".to_string(),
        );
        templates.insert(
            "hypothesis_formed".to_string(),
            "Evidence suggests: {cause}. Confidence: {level}. Grounded in: {witnessed_cases}."
                .to_string(),
        );
        templates.insert(
            "action_taken".to_string(),
            "Applying: {action}.".to_string(),
        );
        templates.insert(
            "fix_verified".to_string(),
            "{target} restored to sattva. Fix: {action}.".to_string(),
        );
        templates.insert(
            "hetvabhasa_triggered".to_string(),
            "Inference halted. Fallacy detected: {type}. Reason: {explanation}.".to_string(),
        );
        templates.insert(
            "unknown_state".to_string(),
            "No vyapti matches this state: {state}. Describe what you see.".to_string(),
        );
        templates.insert(
            "upeksha_applied".to_string(),
            "Severity below threshold. Monitoring only.".to_string(),
        );

        Self { templates }
    }

    pub fn generate(&self, state: AgentState, context: HashMap<&str, &str>) -> String {
        let key = match state {
            AgentState::DiagnosisStarted => "diagnosis_started",
            AgentState::HypothesisFormed => "hypothesis_formed",
            AgentState::ActionTaken => "action_taken",
            AgentState::FixVerified => "fix_verified",
            AgentState::HetvabhasaTriggered => "hetvabhasa_triggered",
            AgentState::UnknownState => "unknown_state",
            AgentState::UpekshaApplied => "upeksha_applied",
        };

        let mut text = self.templates.get(key).unwrap().clone();
        for (k, v) in context {
            text = text.replace(&format!("{{{}}}", k), v);
        }
        text
    }
}
