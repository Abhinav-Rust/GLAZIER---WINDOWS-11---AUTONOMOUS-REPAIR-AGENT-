use crate::wrl::ast::{AbhavaNode, Action};
use crate::knowledge::ontology::AbhavaCategory;

pub struct AbhavaClassifier;

impl AbhavaClassifier {
    pub fn classify(node: &AbhavaNode, category: AbhavaCategory) -> Action {
        match category {
            AbhavaCategory::Pragabhava(_) => node.pragabhava.clone(),
            AbhavaCategory::Dhvamsabhava(_) => node.dhvamsabhava.first().unwrap().clone(), // Simplified
            AbhavaCategory::Atyantabhava(_) => node.atyantabhava.clone(),
            AbhavaCategory::Anyonyabhava(_) => node.anyonyabhava.clone(),
        }
    }
}
