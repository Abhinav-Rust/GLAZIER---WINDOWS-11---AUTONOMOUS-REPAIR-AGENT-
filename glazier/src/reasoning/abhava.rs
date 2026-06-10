use crate::knowledge::ontology::AbhavaCategory;
use crate::wrl::ast::{AbhavaNode, Action};

pub struct AbhavaClassifier;

impl AbhavaClassifier {
    /// Routes the diagnostic logic to the appropriate action based on the *type* of absence observed.
    /// Treating all absences identically is a fallacy; the Vaiśeṣika ontology demands distinction.
    pub fn classify_and_route(node: &AbhavaNode, category: AbhavaCategory) -> Action {
        match category {
            // Prāgabhāva (Antecedent non-existence): The component never existed.
            // Example: A driver has never been installed on this OS instance.
            AbhavaCategory::Pragabhava(_) => {
                println!("TRACE [ABHAVA] Prāgabhāva detected: Component never existed. Routing to fresh install.");
                node.pragabhava.clone()
            }

            // Dhvaṃsābhāva (Destructive non-existence): The component existed but is gone/broken.
            // Example: Driver was uninstalled or its files were corrupted.
            AbhavaCategory::Dhvamsabhava(_) => {
                println!("TRACE [ABHAVA] Dhvaṃsābhāva detected: Component was destroyed/corrupted. Routing to reinstall/rollback.");
                // For simplicity, we just take the first option in the choice list,
                // but a deeper engine would evaluate which choice to use (reinstall vs rollback).
                node.dhvamsabhava.first().unwrap().clone()
            }

            // Atyantābhāva (Absolute non-existence): The component cannot exist here.
            // Example: An Intel driver on an AMD system. Categorical incompatibility.
            AbhavaCategory::Atyantabhava(_) => {
                println!("TRACE [ABHAVA] Atyantābhāva detected: Categorical incompatibility. Routing to alternative.");
                node.atyantabhava.clone()
            }

            // Anyonyābhāva (Mutual non-existence): Wrong category entirely.
            // Example: Looking for an audio driver issue when the hardware itself is unplugged.
            AbhavaCategory::Anyonyabhava(_) => {
                println!("TRACE [ABHAVA] Anyonyābhāva detected: Wrong category. Redirecting diagnosis scope.");
                node.anyonyabhava.clone()
            }
        }
    }
}
