use anyhow::Result;
use glazier::execution::karma::{KarmaExecutor, SandboxMockExecutor};
use glazier::execution::logger::ActionLogger;
use glazier::knowledge::loader::KnowledgeBaseLoader;
use glazier::memory::episodic::EpisodicMemory;
use glazier::memory::working::WorkingMemory;
use glazier::nlp::automaton::IntentAutomaton;
use glazier::nlp::response::{AgentState, ResponseGenerator};
use glazier::nlp::tokeniser::Tokeniser;
use glazier::pramana::pratyaksha::{PratyakshaProvider, SandboxMock};
use glazier::reasoning::anumana::InferenceEngine;
use glazier::reasoning::vyapti::VyaptiStore;

#[tokio::main]
async fn main() -> Result<()> {
    println!("GLAZIER Windows Repair Agent - Starting up...");

    // NLU Setup
    let tokeniser = Tokeniser::new();
    let response_gen = ResponseGenerator::new();

    // Memory Setup
    let db_path = std::env::var("GLAZIER_DB_PATH")
        .unwrap_or_else(|_| "rocksdb://glazier_episodic.db".to_string());
    let episodic = EpisodicMemory::new(&db_path).await?;
    let mut working_memory = WorkingMemory::new();

    // Initialize Episodic Memory with Seeds
    println!("Initializing Episodic Memory...");
    let _ = episodic.init_schema().await;
    if let Ok(seeds_json) = std::fs::read_to_string("seeds.json") {
        if let Ok(seeds) =
            serde_json::from_str::<Vec<glazier::memory::episodic::EpisodicCase>>(&seeds_json)
        {
            for case in seeds {
                let _ = episodic.store_case(case).await;
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
    let mut anumana_blocks = Vec::new();
        let mut virodha_blocks = Vec::new();

    // Dynamically load all .wrl files in the kb/ directory tree
    let walker = walkdir::WalkDir::new("kb").into_iter();
    for entry in walker.filter_map(|e| e.ok()) {
        if entry.path().extension().and_then(|s| s.to_str()) == Some("wrl") {
            let path_str = entry.path().to_string_lossy();
            if let Ok(program) = KnowledgeBaseLoader::load_file(&path_str) {
                for block in program.blocks {
                    if let glazier::wrl::ast::Block::Adhikara(adhikara) = block {
                        for child in adhikara.blocks {
                            match child {
                                glazier::wrl::ast::Block::Vyapti(v) => vyapti_store.add_rule(v),
                                glazier::wrl::ast::Block::Anumana(a) => anumana_blocks.push(a),
                                    glazier::wrl::ast::Block::Virodha(v) => virodha_blocks.push(v),
                                _ => {}
                            }
                        }
                    }
                }
            } else {
                println!("Warning: Failed to parse WRL file: {}", path_str);
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
        println!(
            "Agent: {}",
            response_gen.generate(AgentState::DiagnosisStarted, ctx)
        );

        // 4. Pratyaksha (Observation)
        let obs = pratyaksha.observe_device(&target.to_lowercase())?;
        working_memory
            .observations
            .push(glazier::memory::working::TimestampedObservation {
                timestamp: chrono::Utc::now(),
                observation: obs.clone(),
            });
        println!("Observed State: {:?}", obs);

        // Classify initial system Guṇa state
        let current_guna = glazier::conflict::guna::GunaClassifier::classify_state(&working_memory.observations);
        println!("Initial System State Guṇa: {:?}", current_guna);

        // Upamana Case Matching
        let symptoms = intent.symptom.iter().cloned().collect::<Vec<_>>();
        let matched_case = glazier::pramana::upamana::UpamanaMatcher::match_similar_case(
            &episodic,
            target,
            &symptoms,
        ).await.unwrap_or(None);
        let witness_cases = if let Some(c) = matched_case {
            vec![c.id]
        } else {
            vec!["case_seed_001".to_string()]
        };

        // 5. Reasoning & Execution
        let mut anumana_resolved = false;
        println!("\nAttempting Anumana resolution based on observed state...");
        for anumana in &anumana_blocks {
            if _inference_engine.execute(anumana, &mut working_memory).is_ok() {
                println!("Matched Anumana: {}", anumana.name);

                let action = anumana.nigamana.clone();
                let outcome = karma.execute_action(&action)?;

                let now = chrono::Utc::now();

                // Construct string representations for logging
                let hetu_str = match &anumana.hetu {
                    glazier::wrl::ast::Predicate::State(s) => {
                        let mut arg_strs = Vec::new();
                        for arg in &s.property.args {
                            match arg {
                                glazier::wrl::ast::PropertyArg::Integer(i) => arg_strs.push(i.to_string()),
                                glazier::wrl::ast::PropertyArg::String(st) => arg_strs.push(st.clone()),
                                glazier::wrl::ast::PropertyArg::Ident(id) => arg_strs.push(id.clone()),
                                _ => arg_strs.push("?".to_string())
                            }
                        }
                        format!("{}({})", s.property.name, arg_strs.join(", "))
                    },
                    _ => "?".to_string()
                };

                let action_str = {
                    let mut arg_strs = Vec::new();
                    for arg in &action.args {
                        match arg {
                            glazier::wrl::ast::ActionArg::Integer(i) => arg_strs.push(i.to_string()),
                            glazier::wrl::ast::ActionArg::String(st) => arg_strs.push(st.clone()),
                            glazier::wrl::ast::ActionArg::Ident(id) => arg_strs.push(id.clone()),
                            _ => arg_strs.push("?".to_string())
                        }
                    }
                    format!("{}({})", action.name, arg_strs.join(", "))
                };

                action_logger.log_anumana(
                    &anumana.name,
                    &hetu_str,
                    &now,
                    &anumana.udaharana,
                    &witness_cases,
                    &action_str,
                );
                action_logger.log_action(action.clone(), outcome, glazier::wrl::ast::GunaType::Sattva, true);

                // 6. Response: Fix Verified
                let mut ctx2 = std::collections::HashMap::new();
                ctx2.insert("target", target.as_str());
                ctx2.insert("action", action_str.as_str());
                println!(
                    "\nAgent: {}",
                    response_gen.generate(AgentState::FixVerified, ctx2)
                );

                anumana_resolved = true;
                break;
            }
        }

        if !anumana_resolved {
            println!("No matching Anumana rule found for the observed state.");
            if let Some(virodha) = virodha_blocks.first() {
                println!("\nFallback to Kauṭilya Caturupāya resolution sequence...");
                let outcome = glazier::conflict::upaya::UpayaResolver::resolve(virodha, &karma)?;
                println!("Caturupāya Execution Outcome: {:?}", outcome);
            }
        }
    }

    Ok(())
}
