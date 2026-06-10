use crate::memory::working::WorkingMemory;
use crate::reasoning::hetvabhasa::HetvabhasaGuard;
use crate::reasoning::vyapti::VyaptiStore;
use crate::wrl::ast::AnumanaNode;
use crate::wrl::validator::{HetvabhasaError, Validator};
use anyhow::Result;

pub struct InferenceEngine {
    vyapti_store: VyaptiStore,
    validator: Validator,
}

impl InferenceEngine {
    pub fn new(vyapti_store: VyaptiStore) -> Self {
        Self {
            vyapti_store,
            validator: Validator::new(),
        }
    }

    pub fn execute(
        &self,
        anumana: &AnumanaNode,
        wm: &mut WorkingMemory,
    ) -> Result<(), HetvabhasaError> {
        // 1. Validate structural fallacies (e.g., Prakaranasama) at the syntax level
        self.validator.validate_anumana(anumana)?;

        // 2. Full Hetvabhasa checks against working memory observations
        // This enforces Asiddha, Viruddha, Anaikantika, and Katatita.
        HetvabhasaGuard::check(anumana, wm)?;

        // 3. Fetch Vyapti rule
        let vyapti = self.vyapti_store.get_rule(&anumana.udaharana);
        if vyapti.is_none() {
            return Err(HetvabhasaError {
                anumana_name: anumana.name.clone(),
                fallacy_type: crate::wrl::validator::HetvabhasaType::Anaikantika,
                explanation: format!(
                    "Vyapti rule '{}' not found, unable to ground inference.",
                    anumana.udaharana
                ),
            });
        }

        // Ensure the Vyapti has witnessed cases (Udaharana)
        if vyapti.unwrap().witnessed.is_empty() {
            return Err(HetvabhasaError {
                anumana_name: anumana.name.clone(),
                fallacy_type: crate::wrl::validator::HetvabhasaType::Asiddha,
                explanation: format!(
                    "Vyapti rule '{}' lacks witnessed cases. Rule is unproven.",
                    anumana.udaharana
                ),
            });
        }

        Ok(())
    }
}
