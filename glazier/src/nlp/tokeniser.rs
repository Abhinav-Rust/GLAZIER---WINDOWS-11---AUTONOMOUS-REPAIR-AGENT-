use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum SemanticToken {
    Device(String),
    Symptom(String),
    State(String),
    ErrorCode(i64),
    Context(String),
    Intent(String),
    Literal(String),
}

pub struct Tokeniser {
    patterns: HashMap<String, SemanticToken>,
}

impl Tokeniser {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();
        // Seed patterns
        patterns.insert("mic".to_string(), SemanticToken::Device("Microphone".to_string()));
        patterns.insert("microphone".to_string(), SemanticToken::Device("Microphone".to_string()));
        patterns.insert("audio input".to_string(), SemanticToken::Device("Microphone".to_string()));

        patterns.insert("not working".to_string(), SemanticToken::Symptom("NotFunctioning".to_string()));
        patterns.insert("doesn't work".to_string(), SemanticToken::Symptom("NotFunctioning".to_string()));

        patterns.insert("yellow bang".to_string(), SemanticToken::State("ErrorFlagged".to_string()));
        patterns.insert("error code 43".to_string(), SemanticToken::ErrorCode(43));

        patterns.insert("after update".to_string(), SemanticToken::Context("PostWindowsUpdate".to_string()));

        patterns.insert("fix".to_string(), SemanticToken::Intent("Repair".to_string()));
        patterns.insert("diagnose".to_string(), SemanticToken::Intent("Diagnose".to_string()));
        patterns.insert("check".to_string(), SemanticToken::Intent("Observe".to_string()));

        Self { patterns }
    }

    pub fn tokenise(&self, input: &str) -> Vec<SemanticToken> {
        // Highly simplified greedy tokeniser for demonstration.
        // In reality, this would use an ordered Trie.
        let mut tokens = Vec::new();
        let lower_input = input.to_lowercase();

        // This is a naive substring approach instead of a true Markov rewrite,
        // sufficient for the scope of establishing the architecture.
        for (pattern, token) in &self.patterns {
            if lower_input.contains(pattern) {
                tokens.push(token.clone());
            }
        }

        if tokens.is_empty() {
            tokens.push(SemanticToken::Literal(input.to_string()));
        }

        tokens
    }
}
