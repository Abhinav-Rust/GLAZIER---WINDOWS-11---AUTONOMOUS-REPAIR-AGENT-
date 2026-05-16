use crate::nlp::tokeniser::SemanticToken;

#[derive(Debug)]
pub struct ResolvedIntent {
    pub intent: String,
    pub target: Option<String>,
    pub symptom: Option<String>,
    pub context: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
enum NfaState {
    Start,
    HaveIntent,
    HaveTarget,
    HaveSymptom,
    Complete,
}

pub struct IntentAutomaton;

impl IntentAutomaton {
    pub fn resolve(tokens: &[SemanticToken]) -> ResolvedIntent {
        let mut resolved = ResolvedIntent {
            intent: "Unknown".to_string(),
            target: None,
            symptom: None,
            context: None,
        };

        let mut current_state = NfaState::Start;

        for token in tokens {
            match (&current_state, token) {
                (NfaState::Start, SemanticToken::Intent(i)) => {
                    resolved.intent = i.clone();
                    current_state = NfaState::HaveIntent;
                }
                (NfaState::Start, SemanticToken::Device(d)) => {
                    resolved.target = Some(d.clone());
                    current_state = NfaState::HaveTarget;
                }
                (NfaState::Start, SemanticToken::Symptom(s)) => {
                    resolved.symptom = Some(s.clone());
                    current_state = NfaState::HaveSymptom;
                }
                (NfaState::Start, SemanticToken::ErrorCode(c)) => {
                    resolved.symptom = Some(format!("ErrorCode({})", c));
                    current_state = NfaState::HaveSymptom;
                }
                (NfaState::Start, SemanticToken::Context(c)) => {
                    resolved.context = Some(c.clone());
                    // state remains start as context is auxiliary
                }

                (NfaState::HaveIntent, SemanticToken::Device(d)) => {
                    resolved.target = Some(d.clone());
                    current_state = NfaState::Complete;
                }
                (NfaState::HaveIntent, SemanticToken::Symptom(s)) => {
                    resolved.symptom = Some(s.clone());
                    current_state = NfaState::Complete;
                }
                (NfaState::HaveIntent, SemanticToken::ErrorCode(c)) => {
                    resolved.symptom = Some(format!("ErrorCode({})", c));
                    current_state = NfaState::Complete;
                }
                (NfaState::HaveTarget, SemanticToken::Intent(i)) => {
                    resolved.intent = i.clone();
                    current_state = NfaState::Complete;
                }
                (NfaState::HaveTarget, SemanticToken::Symptom(s)) => {
                    resolved.symptom = Some(s.clone());
                    current_state = NfaState::Complete;
                }
                (NfaState::HaveTarget, SemanticToken::ErrorCode(c)) => {
                    resolved.symptom = Some(format!("ErrorCode({})", c));
                    current_state = NfaState::Complete;
                }
                (NfaState::HaveSymptom, SemanticToken::Device(d)) => {
                    resolved.target = Some(d.clone());
                    current_state = NfaState::Complete;
                }
                (NfaState::HaveSymptom, SemanticToken::Intent(i)) => {
                    resolved.intent = i.clone();
                    current_state = NfaState::Complete;
                }

                // Keep absorbing missing parts if we hit complete early but get more info
                (NfaState::Complete, SemanticToken::Device(d)) => {
                    if resolved.target.is_none() { resolved.target = Some(d.clone()); }
                }
                (NfaState::Complete, SemanticToken::Symptom(s)) => {
                     if resolved.symptom.is_none() { resolved.symptom = Some(s.clone()); }
                }
                (NfaState::Complete, SemanticToken::ErrorCode(c)) => {
                     if resolved.symptom.is_none() { resolved.symptom = Some(format!("ErrorCode({})", c)); }
                }
                (NfaState::Complete, SemanticToken::Context(c)) => {
                     if resolved.context.is_none() { resolved.context = Some(c.clone()); }
                }

                // Absorb context at any non-start state without changing state
                (_, SemanticToken::Context(c)) => {
                    resolved.context = Some(c.clone());
                }

                // Ignore literals and state transitions not explicitly covered
                _ => {}
            }
        }

        resolved
    }
}
