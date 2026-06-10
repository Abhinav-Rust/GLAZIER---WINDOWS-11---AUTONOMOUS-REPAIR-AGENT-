use crate::nlp::trie::OrderedTrie;

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
    trie: OrderedTrie<SemanticToken>,
}

impl Tokeniser {
    pub fn new() -> Self {
        let mut trie = OrderedTrie::new();
        // Seed patterns
        trie.insert(
            "microphone",
            SemanticToken::Device("Microphone".to_string()),
        );
        trie.insert("mic", SemanticToken::Device("Microphone".to_string()));
        trie.insert(
            "audio input",
            SemanticToken::Device("Microphone".to_string()),
        );

        trie.insert(
            "not working",
            SemanticToken::Symptom("NotFunctioning".to_string()),
        );
        trie.insert(
            "doesn't work",
            SemanticToken::Symptom("NotFunctioning".to_string()),
        );
        trie.insert(
            "stopped working",
            SemanticToken::Symptom("NotFunctioning".to_string()),
        );

        trie.insert(
            "yellow bang",
            SemanticToken::State("ErrorFlagged".to_string()),
        );
        trie.insert("error code 43", SemanticToken::ErrorCode(43));

        trie.insert(
            "after update",
            SemanticToken::Context("PostWindowsUpdate".to_string()),
        );

        trie.insert("fix", SemanticToken::Intent("Repair".to_string()));
        trie.insert("diagnose", SemanticToken::Intent("Diagnose".to_string()));
        trie.insert("check", SemanticToken::Intent("Observe".to_string()));

        Self { trie }
    }

    pub fn tokenise(&self, input: &str) -> Vec<SemanticToken> {
        let mut tokens = Vec::new();
        let lower_input = input.to_lowercase();
        let chars: Vec<char> = lower_input.chars().collect();
        let mut idx = 0;

        while idx < chars.len() {
            // Skip whitespace
            if chars[idx].is_whitespace() {
                idx += 1;
                continue;
            }

            if let Some((next_idx, token)) = self.trie.find_longest_match(&lower_input, idx) {
                tokens.push(token);
                idx = next_idx;
            } else {
                // If no match, we advance by one character (in reality, we'd chunk by words for Literals)
                // For simplicity of this strict transducer, we'll collect unrecognized words into Literals.
                let mut unrec_end = idx;
                while unrec_end < chars.len() && !chars[unrec_end].is_whitespace() {
                    // check if a trie match starts here to break early? (simplified here)
                    unrec_end += 1;
                }

                let literal_str: String = chars[idx..unrec_end].iter().collect();
                tokens.push(SemanticToken::Literal(literal_str));
                idx = unrec_end;
            }
        }

        tokens
    }
}
