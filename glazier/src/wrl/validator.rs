use super::ast::*;

#[derive(Debug, PartialEq)]
pub enum HetvabhasaType {
    Asiddha,       // hetu not confirmed by pratyaksha
    Viruddha,      // hetu contradicts nigamana
    Anaikantika,   // hetu matches multiple vyaptis
    Katatita,      // observation timestamp > staleness_threshold
    Prakaranasama, // nigamana references its own hetu
}

#[derive(Debug)]
pub struct HetvabhasaError {
    pub anumana_name: String,
    pub fallacy_type: HetvabhasaType,
    pub explanation: String,
}

pub struct Validator;

impl Validator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_anumana(&self, anumana: &AnumanaNode) -> Result<(), HetvabhasaError> {
        // Prakaranasama Check (circular reasoning):
        // If the nigamana action name exactly matches the hetu property name (rudimentary check).
        if let Predicate::State(state) = &anumana.hetu {
            if anumana.nigamana.name == state.property.name {
                return Err(HetvabhasaError {
                    anumana_name: anumana.name.clone(),
                    fallacy_type: HetvabhasaType::Prakaranasama,
                    explanation: format!(
                        "Nigamana action '{}' references its own hetu property.",
                        anumana.nigamana.name
                    ),
                });
            }
        }

        // At this layer, without working memory, we only check structural Prakaranasama.
        // Asiddha, Viruddha, Anaikantika, and Katatita are runtime fallacies checked against WorkingMemory.
        // We will perform those checks in the reasoning engine during the inference cycle.

        Ok(())
    }
}
