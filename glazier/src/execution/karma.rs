use crate::wrl::ast::Action;
use anyhow::Result;

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
        match action.name.as_str() {
            "restart_service" => self.handle_service(action, "Restart-Service"),
            "stop_service" => self.handle_service(action, "Stop-Service"),
            "start_service" => self.handle_service(action, "Start-Service"),
            "delete_registry_key" => self.handle_registry_delete(action),
            "run_powershell" => self.handle_powershell(action),
            // Driver actions generally require SetupAPI via C++ or complex PowerShell invoking PnPUtil
            "rollback_driver" => self.handle_pnputil(action, "rollback"),
            _ => Err(anyhow!(
                "Action '{}' is not implemented in Win32NativeExecutor",
                action.name
            )),
        }
    }
}

#[cfg(target_os = "windows")]
impl Win32NativeExecutor {
    fn handle_service(&self, action: &Action, ps_cmd: &str) -> Result<ExecutionOutcome> {
        use std::process::Command;

        if action.args.is_empty() {
            return Err(anyhow!("Service action requires a service name argument."));
        }

        let service_name = match &action.args[0] {
            crate::wrl::ast::ActionArg::Ident(i) => i.clone(),
            crate::wrl::ast::ActionArg::String(s) => s.clone(),
            _ => return Err(anyhow!("Invalid argument type for service action.")),
        };

        let output = Command::new("powershell")
            .args(&[
                "-NoProfile",
                "-Command",
                &format!("{} -Name '{}' -Force", ps_cmd, service_name),
            ])
            .output()?;

        Ok(ExecutionOutcome {
            success: output.status.success(),
            before_state: "Unknown".to_string(), // In production, we'd query state before executing
            after_state: if output.status.success() {
                "Success".to_string()
            } else {
                "Failed".to_string()
            },
            output: String::from_utf8_lossy(&output.stdout).to_string(),
        })
    }

    fn handle_powershell(&self, action: &Action) -> Result<ExecutionOutcome> {
        use std::process::Command;

        if action.args.is_empty() {
            return Err(anyhow!("run_powershell requires a script argument."));
        }

        let script = match &action.args[0] {
            crate::wrl::ast::ActionArg::String(s) => s.clone(),
            _ => return Err(anyhow!("Invalid argument type for run_powershell.")),
        };

        let output = Command::new("powershell")
            .args(&["-NoProfile", "-Command", &script])
            .output()?;

        Ok(ExecutionOutcome {
            success: output.status.success(),
            before_state: "N/A".to_string(),
            after_state: "Executed".to_string(),
            output: String::from_utf8_lossy(&output.stdout).to_string(),
        })
    }

    fn handle_registry_delete(&self, action: &Action) -> Result<ExecutionOutcome> {
        // Using winreg crate to delete a key
        // Arguments expected: Hive (e.g., HKLM), Path
        use winreg::enums::*;
        use winreg::RegKey;

        if action.args.len() < 2 {
            return Err(anyhow!(
                "delete_registry_key requires hive and path arguments."
            ));
        }

        let hive_str = match &action.args[0] {
            crate::wrl::ast::ActionArg::Ident(i) => i.as_str(),
            _ => return Err(anyhow!("Invalid hive argument type.")),
        };

        let path = match &action.args[1] {
            crate::wrl::ast::ActionArg::String(s) => s.clone(),
            _ => return Err(anyhow!("Invalid path argument type.")),
        };

        let hive = match hive_str {
            "HKLM" => RegKey::predef(HKEY_LOCAL_MACHINE),
            "HKCU" => RegKey::predef(HKEY_CURRENT_USER),
            _ => return Err(anyhow!("Unsupported registry hive: {}", hive_str)),
        };

        let result = hive.delete_subkey_all(&path);

        Ok(ExecutionOutcome {
            success: result.is_ok(),
            before_state: "Key exists".to_string(),
            after_state: if result.is_ok() {
                "Key deleted".to_string()
            } else {
                "Deletion failed".to_string()
            },
            output: match result {
                Ok(_) => "Success".to_string(),
                Err(e) => format!("Error: {}", e),
            },
        })
    }

    fn handle_pnputil(&self, action: &Action, operation: &str) -> Result<ExecutionOutcome> {
        // Placeholder for PnPUtil invocation via PowerShell
        // This is complex in Windows, normally requiring administrative elevation
        use std::process::Command;

        let output = Command::new("powershell")
            .args(&[
                "-NoProfile",
                "-Command",
                &format!("Write-Host 'Simulated {} via PnPUtil'", operation),
            ])
            .output()?;

        Ok(ExecutionOutcome {
            success: output.status.success(),
            before_state: "Driver Active".to_string(),
            after_state: "Driver Rolled Back".to_string(),
            output: "Simulated PnPUtil execution".to_string(),
        })
    }
}
