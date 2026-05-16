use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TrieNode<T: Clone> {
    children: HashMap<char, TrieNode<T>>,
    pub value: Option<T>,
}

impl<T: Clone> TrieNode<T> {
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
            value: None,
        }
    }
}

pub struct OrderedTrie<T: Clone> {
    root: TrieNode<T>,
}

impl<T: Clone> OrderedTrie<T> {
    pub fn new() -> Self {
        Self { root: TrieNode::new() }
    }

    pub fn insert(&mut self, key: &str, value: T) {
        let mut current = &mut self.root;
        for c in key.chars() {
            current = current.children.entry(c).or_insert(TrieNode::new());
        }
        current.value = Some(value);
    }

    pub fn find_longest_match(&self, input: &str, start_idx: usize) -> Option<(usize, T)> {
        let mut current = &self.root;
        let mut best_match = None;
        let mut current_idx = start_idx;

        let chars: Vec<char> = input.chars().collect();

        while current_idx < chars.len() {
            let c = chars[current_idx];
            if let Some(child) = current.children.get(&c) {
                current = child;
                if let Some(val) = &current.value {
                    best_match = Some((current_idx + 1, val.clone()));
                }
                current_idx += 1;
            } else {
                break;
            }
        }

        best_match
    }
}
