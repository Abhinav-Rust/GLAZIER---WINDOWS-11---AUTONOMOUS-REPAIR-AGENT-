use crate::wrl::ast::TarkaNode;

pub enum TarkaResult {
    Confirmed,
    Contradicted,
}

pub struct TarkaTester;

impl TarkaTester {
    // In a real system, this would execute the test action and return the result.
    pub fn evaluate_hypothesis(node: &TarkaNode, result: TarkaResult) -> crate::wrl::ast::Action {
        match result {
            TarkaResult::Confirmed => node.if_confirmed_action.clone(),
            TarkaResult::Contradicted => node.if_contradicted_action.clone(),
        }
    }
}
