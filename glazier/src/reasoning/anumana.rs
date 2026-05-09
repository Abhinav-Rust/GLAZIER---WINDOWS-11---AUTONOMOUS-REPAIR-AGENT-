use anyhow::Result;
use crate::wrl::ast::AnumanaNode;
use crate::memory::working::WorkingMemory;
use crate::reasoning::vyapti::VyaptiStore;
use crate::wrl::validator::{Validator, HetvabhasaError};

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

    pub fn execute(&self, anumana: &AnumanaNode, _wm: &mut WorkingMemory) -> Result<(), HetvabhasaError> {
        // 1. Validate structural fallacies (e.g., Prakaranasama)
        self.validator.validate_anumana(anumana)?;

        // 2. Fetch Vyapti rule
        let vyapti = self.vyapti_store.get_rule(&anumana.udaharana);
        if vyapti.is_none() {
            return Err(HetvabhasaError {
                anumana_name: anumana.name.clone(),
                fallacy_type: crate::wrl::validator::HetvabhasaType::Anaikantika,
                explanation: format!("Vyapti rule '{}' not found", anumana.udaharana),
            });
        }

        // Note: Full Hetvabhasa checks against working memory observations
        // like Asiddha (hetu not observed) and Katatita (stale observation)
        // should be implemented here in a real scenario.

        Ok(())
    }
}
