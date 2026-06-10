use serde::{Deserialize, Serialize};

// Vaiśeṣika Padārtha (Categories of Being) mapping in Rust

/// Dravya (Substance) - the fundamental entities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Dravya {
    HardwareDevice(String),    // e.g., Microphone
    SoftwareComponent(String), // e.g., AudioSrv
    SystemResource(String),    // e.g., RegistryKey
}

/// Guna (Quality) - attributes or states of substances
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GunaAttribute {
    State(String),  // e.g., "stopped", "running"
    ErrorCode(i64), // e.g., 43
    Version(String),
}

/// Karma (Action) - operations or transformations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Karma {
    RestartService(String),
    StopService(String),
    RollbackDriver(String),
    ReinstallDriver(String),
    RunCommand(String),
}

/// Samanya (Universality) - shared characteristics / Vyapti categories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Samanya(pub String);

/// Visesa (Particularity) - unique instances or specific UUIDs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Visesa(pub String);

/// Samavaya (Inherence) - structural dependencies (e.g., driver inheres in device)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Samavaya {
    pub container: Dravya,
    pub contained: Dravya,
}

/// Abhava (Non-existence) - the four types of absence
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AbhavaCategory {
    Pragabhava(String),   // Antecedent non-existence (never installed)
    Dhvamsabhava(String), // Destructive non-existence (uninstalled or broken)
    Atyantabhava(String), // Absolute non-existence (incompatible)
    Anyonyabhava(String), // Mutual non-existence (wrong category)
}
