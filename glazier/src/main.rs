use anyhow::Result;
use glazier::memory::working::WorkingMemory;
use glazier::memory::episodic::EpisodicMemory;
use glazier::nlp::tokeniser::Tokeniser;
use glazier::nlp::automaton::IntentAutomaton;
use glazier::nlp::response::{ResponseGenerator, AgentState};
use glazier::pramana::pratyaksha::{PratyakshaProvider, SandboxMock};
use glazier::execution::karma::{KarmaExecutor, SandboxMockExecutor};
use glazier::execution::logger::ActionLogger;
use glazier::reasoning::vyapti::VyaptiStore;
use glazier::reasoning::anumana::InferenceEngine;
use glazier::knowledge::loader::KnowledgeBaseLoader;

#[tokio::main]
async fn main() -> Result<()> {
    println!("GLAZIER Windows Repair Agent - Starting up...");

    // NLU Setup
    let tokeniser = Tokeniser::new();
    let response_gen = ResponseGenerator::new();

    // Memory Setup
    let db_path = std::env::var("GLAZIER_DB_PATH").unwrap_or_else(|_| "rocksdb://glazier_episodic.db".to_string());
    let _episodic = EpisodicMemory::new(&db_path).await?;
    let mut working_memory = WorkingMemory::new();

    // Initialize Episodic Memory with Seeds
    println!("Initializing Episodic Memory...");
    let _ = _episodic.init_schema().await;
    if let Ok(seeds_json) = std::fs::read_to_string("seeds.json") {
        if let Ok(seeds) = serde_json::from_str::<Vec<glazier::memory::episodic::EpisodicCase>>(&seeds_json) {
            for case in seeds {
                let _ = _episodic.store_case(case).await;
            }
            println!("Seeds loaded successfully.");
        }
    } else {
        println!("No seeds.json found. Skipping seed initialization.");
    }

    // Provider Setup
    let pratyaksha = SandboxMock;
    let karma = SandboxMockExecutor;
    let mut action_logger = ActionLogger::new();

    // Load KB
    println!("Loading Knowledge Base...");
    let mut vyapti_store = VyaptiStore::new();
    if let Ok(program) = KnowledgeBaseLoader::load_file("kb/audio/microphone.wrl") {
        for block in program.blocks {
            if let glazier::wrl::ast::Block::Adhikara(adhikara) = block {
                for child in adhikara.blocks {
                    if let glazier::wrl::ast::Block::Vyapti(v) = child {
                        vyapti_store.add_rule(v);
                    }
                }
            }
        }
    }

    let _inference_engine = InferenceEngine::new(vyapti_store);

    // Simulate an input loop (single iteration for demonstration)
    let input = "mic doesn't work after update, error code 43";
    println!("\nUser Input: \"{}\"", input);

    // 1. NLU Tokenise
    let tokens = tokeniser.tokenise(input);
    println!("Tokens: {:?}", tokens);

    // 2. Resolve Intent
    let intent = IntentAutomaton::resolve(&tokens);
    println!("Resolved Intent: {:?}", intent);

    // 3. Response: Diagnosis Started
    if let Some(target) = &intent.target {
        let mut ctx = std::collections::HashMap::new();
        ctx.insert("target", target.as_str());
        ctx.insert("properties", "device state, error codes");
        println!("Agent: {}", response_gen.generate(AgentState::DiagnosisStarted, ctx));

        // 4. Pratyaksha (Observation)
        let obs = pratyaksha.observe_device(&target.to_lowercase())?;
        working_memory.observations.push(glazier::memory::working::TimestampedObservation {
            timestamp: chrono::Utc::now(),
            observation: obs.clone(),
        });
        println!("Observed State: {:?}", obs);

        // 5. Reasoning & Execution placeholder (simulating Anumana trigger)
        // In full integration, the engine would find the matching Anumana from the Adhikara
        // and invoke inference_engine.execute(anumana, &mut working_memory)
        println!("\nSimulating Anumana resolution based on observed Error Code 43...");

        let action = glazier::wrl::ast::Action {
            name: "rollback_driver".to_string(),
            args: vec![glazier::wrl::ast::ActionArg::Ident("realtek_audio".to_string())],
        };

        let outcome = karma.execute_action(&action)?;
        action_logger.log(action, outcome, glazier::wrl::ast::GunaType::Sattva, true);

        // 6. Response: Fix Verified
        let mut ctx2 = std::collections::HashMap::new();
        ctx2.insert("target", target.as_str());
        ctx2.insert("action", "rollback_driver(realtek_audio)");
        println!("\nAgent: {}", response_gen.generate(AgentState::FixVerified, ctx2));
    }

    Ok(())
}
