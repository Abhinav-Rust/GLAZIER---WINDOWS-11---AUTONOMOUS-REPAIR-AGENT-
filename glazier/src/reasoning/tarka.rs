use crate::wrl::ast::{TarkaNode, Action};

pub enum TarkaTestResult {
    Confirmed,
    Contradicted,
}

pub struct TarkaHypothesisLoop;

impl TarkaHypothesisLoop {
    /// Executes the Tarka abductive reasoning loop when no Vyāpti matches.
    pub fn execute(node: &TarkaNode, test_result: TarkaTestResult) -> Action {
        println!("TRACE [TARKA] Commencing abductive hypothesis test.");
        println!("  Assume:  {}", node.assume);
        println!("  Predict: {}", node.predict);
        println!("  Test:    Execution of {}", node.test.name);

        match test_result {
            TarkaTestResult::Confirmed => {
                println!("TRACE [TARKA] Test Confirmed. Establishing: {}. Routing to next action.", node.if_confirmed_establish);
                node.if_confirmed_action.clone()
            },
            TarkaTestResult::Contradicted => {
                println!("TRACE [TARKA] Test Contradicted. Rejecting: {}. Routing to alternative path.", node.if_contradicted_reject);
                node.if_contradicted_action.clone()
            },
        }
    }
}
