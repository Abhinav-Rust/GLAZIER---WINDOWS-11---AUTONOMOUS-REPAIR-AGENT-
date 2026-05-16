use crate::wrl::ast::Action;
use crate::execution::karma::ExecutionOutcome;
use chrono::Utc;

pub struct ActionLogEntry {
    pub timestamp: chrono::DateTime<Utc>,
    pub action: Action,
    pub outcome: ExecutionOutcome,
    pub guna_result: crate::wrl::ast::GunaType,
    pub rollback_available: bool,
}

pub struct ActionLogger {
    logs: Vec<ActionLogEntry>,
}

impl ActionLogger {
    pub fn new() -> Self {
        Self { logs: Vec::new() }
    }

    pub fn log(&mut self, action: Action, outcome: ExecutionOutcome, guna: crate::wrl::ast::GunaType, rollback: bool) {
        let entry = ActionLogEntry {
            timestamp: Utc::now(),
            action: action.clone(),
            outcome: outcome.clone(),
            guna_result: guna.clone(),
            rollback_available: rollback,
        };

        println!("ACTION [{}]", action.name);
        println!("  before_state: {}", outcome.before_state);
        println!("  action_taken: Execution of {}", action.name);
        println!("  after_state:  {}", outcome.after_state);
        println!("  outcome:      {:?}", guna);
        println!("  rollback_available: {}", if rollback { "yes" } else { "no" });

        self.logs.push(entry);
    }
}
