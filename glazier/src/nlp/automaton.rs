use crate::nlp::tokeniser::SemanticToken;

#[derive(Debug)]
pub struct ResolvedIntent {
    pub intent: String,
    pub target: Option<String>,
    pub symptom: Option<String>,
    pub context: Option<String>,
}

enum State {
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

        // Simplified NFA resolution.
        for token in tokens {
            match token {
                SemanticToken::Intent(i) => resolved.intent = i.clone(),
                SemanticToken::Device(d) => resolved.target = Some(d.clone()),
                SemanticToken::Symptom(s) => resolved.symptom = Some(s.clone()),
                SemanticToken::Context(c) => resolved.context = Some(c.clone()),
                _ => {}
            }
        }

        resolved
    }
}
