use crate::wrl::ast::Program;
use crate::wrl::parser::parse_wrl;
use anyhow::{Context, Result};
use std::fs;

pub struct KnowledgeBaseLoader;

impl KnowledgeBaseLoader {
    pub fn load_file(path: &str) -> Result<Program> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read WRL file at {}", path))?;
        parse_wrl(&content).with_context(|| format!("Failed to parse WRL file at {}", path))
    }
}
