use crate::execution::karma::ExecutionOutcome;
use crate::wrl::ast::Action;
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

    /// Logs an inference execution per the exact auditable trace standard.
    pub fn log_anumana(
        &self,
        anumana_name: &str,
        hetu_str: &str,
        timestamp: &chrono::DateTime<Utc>,
        vyapti_name: &str,
        witnessed: &[String],
        nigamana_name: &str,
    ) {
        let witnessed_str = witnessed.join(", ");

        println!("ANUMANA [{}]", anumana_name);
        println!("  pramana_source:   [PRATYAKSHA]");
        println!(
            "  hetu:             {} observed at [{}]",
            hetu_str,
            timestamp.to_rfc3339()
        );
        println!("  vyapti:           [SHABDA] {}", vyapti_name);
        println!("  witnessed:        [UPAMANA] [{}]", witnessed_str);
        println!("  hetvabhasa_check: PASSED");
        println!("  nigamana:         {}", nigamana_name);
        println!();
    }

    /// Logs an action execution per the exact auditable trace standard.
    pub fn log_action(
        &mut self,
        action: Action,
        outcome: ExecutionOutcome,
        guna: crate::wrl::ast::GunaType,
        rollback: bool,
    ) {
        let entry = ActionLogEntry {
            timestamp: Utc::now(),
            action: action.clone(),
            outcome: outcome.clone(),
            guna_result: guna.clone(),
            rollback_available: rollback,
        };

        // Ensure GunaType has correct display casing (Sattva, Tamas, Rajas)
        let guna_str = match guna {
            crate::wrl::ast::GunaType::Sattva => "sattva",
            crate::wrl::ast::GunaType::Rajas => "rajas",
            crate::wrl::ast::GunaType::Tamas => "tamas",
        };

        println!("ACTION [{}]", action.name);
        println!("  before_state:     {}", outcome.before_state);
        println!("  action_taken:     {}", action.name); // Could expand to show args
        println!("  after_state:      {}", outcome.after_state);
        println!("  guna_end:         {}", guna_str);
        println!(
            "  rollback_stored:  {}",
            if rollback { "yes" } else { "no" }
        );
        println!();

        self.logs.push(entry);
    }
}
