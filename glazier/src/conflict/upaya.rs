use crate::execution::karma::{ExecutionOutcome, KarmaExecutor};
use crate::wrl::ast::{UpayaType, VirodhaNode};
use anyhow::Result;

pub struct UpayaResolver;

impl UpayaResolver {
    /// Executes the Kauṭilya Caturupāya resolution sequence.
    /// It must attempt resolution strictly in order: Sāma -> Dāna -> Bheda -> Daṇḍa.
    /// Reaching Daṇḍa means all gentler approaches have failed.
    pub fn resolve(
        virodha: &VirodhaNode,
        executor: &dyn KarmaExecutor,
    ) -> Result<ExecutionOutcome> {
        println!(
            "TRACE [VIRODHA] Commencing Caturupāya resolution sequence for conflict: {}",
            virodha.name
        );

        let target_sequence = [
            UpayaType::Sama,
            UpayaType::Dana,
            UpayaType::Bheda,
            UpayaType::Danda,
        ];
        let mut last_outcome = None;

        for expected_type in target_sequence {
            if let Some(item) = virodha.upaya.iter().find(|i| i.upaya_type == expected_type) {
                let phase_name = match expected_type {
                    UpayaType::Sama => "Sāma (Conciliation)",
                    UpayaType::Dana => "Dāna (Provision/Concession)",
                    UpayaType::Bheda => "Bheda (Division/Isolation)",
                    UpayaType::Danda => "Daṇḍa (Force/Override)",
                };

                println!("  Attempting Upāya Phase: {}", phase_name);
                println!("  Executing Action: {}", item.action.name);

                let outcome = executor.execute_action(&item.action)?;
                last_outcome = Some(outcome.clone());

                // If successful, we break the sequence and return.
                // We do not proceed to harsher measures if gentler ones succeed.
                if outcome.success {
                    println!("  Phase {} succeeded. Halting escalation.", phase_name);
                    return Ok(outcome);
                } else {
                    println!("  Phase {} failed. Escalating.", phase_name);
                }
            }
        }

        if let Some(outcome) = last_outcome {
            Ok(outcome)
        } else {
            Err(anyhow::anyhow!(
                "No Upāya actions defined in Virodha block."
            ))
        }
    }
}
