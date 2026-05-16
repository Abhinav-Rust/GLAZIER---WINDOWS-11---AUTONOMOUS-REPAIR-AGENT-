use crate::wrl::ast::GunaType;

pub struct GunaClassifier;

impl GunaClassifier {
    pub fn classify_state(symptoms: &[String]) -> GunaType {
        // Simple heuristic:
        if symptoms.is_empty() {
            GunaType::Sattva // Balanced / Working
        } else if symptoms.iter().any(|s| s.contains("conflict") || s.contains("error")) {
            GunaType::Rajas // Active conflict / instability
        } else {
            GunaType::Tamas // Stagnant / dead / service stopped
        }
    }
}
