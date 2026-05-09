use anyhow::Result;
use crate::wrl::ast::Action;

pub trait KarmaExecutor {
    fn execute_action(&self, action: &Action) -> Result<ExecutionOutcome>;
}

#[derive(Debug, Clone)]
pub struct ExecutionOutcome {
    pub success: bool,
    pub before_state: String,
    pub after_state: String,
    pub output: String,
}

#[cfg(not(target_os = "windows"))]
pub struct SandboxMockExecutor;

#[cfg(not(target_os = "windows"))]
impl KarmaExecutor for SandboxMockExecutor {
    fn execute_action(&self, action: &Action) -> Result<ExecutionOutcome> {
        println!("TRACE [KARMA MOCK] Executing action: {}", action.name);

        Ok(ExecutionOutcome {
            success: true,
            before_state: "Mock Before State".to_string(),
            after_state: "Mock After State".to_string(),
            output: format!("Simulated successful execution of {}", action.name),
        })
    }
}

#[cfg(target_os = "windows")]
pub struct Win32NativeExecutor;

#[cfg(target_os = "windows")]
impl KarmaExecutor for Win32NativeExecutor {
    fn execute_action(&self, action: &Action) -> Result<ExecutionOutcome> {
        unimplemented!("Real Windows execution not yet implemented")
    }
}
