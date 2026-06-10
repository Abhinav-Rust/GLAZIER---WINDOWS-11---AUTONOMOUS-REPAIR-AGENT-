use crate::wrl::ast::VyaptiNode;
use std::collections::HashMap;

pub struct VyaptiStore {
    rules: HashMap<String, VyaptiNode>,
}

impl VyaptiStore {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    pub fn add_rule(&mut self, rule: VyaptiNode) {
        self.rules.insert(rule.name.clone(), rule);
    }

    pub fn get_rule(&self, name: &str) -> Option<&VyaptiNode> {
        self.rules.get(name)
    }
}
