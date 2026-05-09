use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Program {
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Block {
    Adhikara(AdhikaraNode),
    Pratyaksha(PratyakshaNode),
    Vyapti(VyaptiNode),
    Anumana(AnumanaNode),
    Abhava(AbhavaNode),
    Tarka(TarkaNode),
    Virodha(VirodhaNode),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdhikaraNode {
    pub name: String,
    pub target: Component,
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Component {
    pub name: String,
    pub param: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PratyakshaNode {
    pub observe: Vec<Property>,
    pub via: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Property {
    pub name: String,
    pub args: Vec<PropertyArg>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PropertyArg {
    String(String),
    Integer(i64),
    Ident(String),
    Action(Box<Action>),
    Named(String, Box<PropertyArg>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VyaptiNode {
    pub name: String,
    pub whenever: Vec<(ConditionOp, Predicate)>, // first op is None essentially (default AND)
    pub always: String, // cause
    pub witnessed: Vec<String>,
    pub confidence: Confidence,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConditionOp {
    None,
    And,
    Or,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Predicate {
    State(StatePredicate),
    Temporal(TemporalContext),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StatePredicate {
    pub property: Property,
    pub expected_state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemporalContext {
    pub event: String,
    pub within: String, // e.g., "72h"
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Confidence {
    Established,
    Provisional,
    Suspected,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnumanaNode {
    pub name: String,
    pub pratijnha: String,
    pub hetu: Predicate,
    pub udaharana: String,
    pub upanaya: String,
    pub nigamana: Action,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Action {
    pub name: String,
    pub args: Vec<ActionArg>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ActionArg {
    Action(Box<Action>),
    String(String),
    Integer(i64),
    Ident(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AbhavaNode {
    pub name: String,
    pub pragabhava: Action,
    pub dhvamsabhava: Vec<Action>,
    pub atyantabhava: Action,
    pub anyonyabhava: Action,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TarkaNode {
    pub name: String,
    pub assume: String,
    pub predict: String,
    pub test: Action,
    pub if_contradicted_reject: String,
    pub if_contradicted_action: Action,
    pub if_confirmed_establish: String,
    pub if_confirmed_action: Action,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VirodhaNode {
    pub name: String,
    pub improving: String,
    pub degrades: String,
    pub guna: GunaType,
    pub upaya: Vec<UpayaItem>,
    pub upeksha_condition: Option<Vec<(ConditionOp, Predicate)>>,
    pub maya_path: Option<Action>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GunaType {
    Tamas,
    Rajas,
    Sattva,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpayaItem {
    pub upaya_type: UpayaType,
    pub action: Action,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UpayaType {
    Sama,
    Dana,
    Bheda,
    Danda,
}
