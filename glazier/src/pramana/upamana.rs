use crate::memory::episodic::{EpisodicCase, EpisodicMemory};
use anyhow::Result;

pub struct UpamanaMatcher;

impl UpamanaMatcher {
    pub async fn match_similar_case(
        memory: &EpisodicMemory,
        target: &str,
        current_symptoms: &[String],
    ) -> Result<Option<EpisodicCase>> {
        println!(
            "TRACE [UPAMANA] Attempting analogical match for target '{}'",
            target
        );

        let cases = memory.find_cases_by_target(target).await?;
        if cases.is_empty() {
            return Ok(None);
        }

        let mut best_case = None;
        let mut best_score = 0.0;

        for case in cases {
            if current_symptoms.is_empty() {
                best_case = Some(case);
                break;
            }

            let mut match_count = 0;
            for sym in current_symptoms {
                if case.symptoms.iter().any(|s| s.to_lowercase().contains(&sym.to_lowercase())) {
                    match_count += 1;
                }
            }
            let score = match_count as f64 / current_symptoms.len() as f64;
            if score > best_score {
                best_score = score;
                best_case = Some(case);
            }
        }

        if let Some(ref matched) = best_case {
            println!(
                "TRACE [UPAMANA] Analogical match found: case_id='{}', target='{}'",
                matched.id, matched.target
            );
        }

        Ok(best_case)
    }
}
