use crate::memory::working::ObservationData;
use anyhow::Result;

pub trait PratyakshaProvider {
    fn observe_device(&self, device_id: &str) -> Result<ObservationData>;
    fn observe_service(&self, service_name: &str) -> Result<ObservationData>;
    fn observe_event_log(
        &self,
        source: &str,
        time_window_hours: u32,
    ) -> Result<Vec<ObservationData>>;
    fn observe_wmi(&self, query: &str) -> Result<Vec<ObservationData>>;
}

#[cfg(not(target_os = "windows"))]
pub struct SandboxMock;

#[cfg(not(target_os = "windows"))]
impl PratyakshaProvider for SandboxMock {
    fn observe_device(&self, device_id: &str) -> Result<ObservationData> {
        println!("TRACE [PRATYAKSHA MOCK] observing device: {}", device_id);
        if device_id == "microphone" {
            Ok(ObservationData::DeviceState {
                name: device_id.to_string(),
                status: "Error".to_string(),
                error_code: Some(43),
            })
        } else {
            Ok(ObservationData::DeviceState {
                name: device_id.to_string(),
                status: "OK".to_string(),
                error_code: None,
            })
        }
    }

    fn observe_service(&self, service_name: &str) -> Result<ObservationData> {
        println!(
            "TRACE [PRATYAKSHA MOCK] observing service: {}",
            service_name
        );
        if service_name == "AudioSrv" {
            Ok(ObservationData::ServiceState {
                name: service_name.to_string(),
                status: "stopped".to_string(),
            })
        } else {
            Ok(ObservationData::ServiceState {
                name: service_name.to_string(),
                status: "running".to_string(),
            })
        }
    }

    fn observe_event_log(
        &self,
        source: &str,
        _time_window_hours: u32,
    ) -> Result<Vec<ObservationData>> {
        println!(
            "TRACE [PRATYAKSHA MOCK] observing event log source: {}",
            source
        );
        Ok(vec![])
    }

    fn observe_wmi(&self, query: &str) -> Result<Vec<ObservationData>> {
        println!("TRACE [PRATYAKSHA MOCK] observing wmi query: {}", query);
        Ok(vec![])
    }
}

#[cfg(target_os = "windows")]
pub struct Win32Native;

#[cfg(target_os = "windows")]
impl PratyakshaProvider for Win32Native {
    fn observe_device(&self, device_id: &str) -> Result<ObservationData> {
        // Use WMI Win32_PnPEntity as a proxy for Device Manager state
        use serde::Deserialize;
        use wmi::{COMLibrary, WMIConnection};

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "PascalCase")]
        struct Win32_PnPEntity {
            name: Option<String>,
            status: Option<String>,
            config_manager_error_code: Option<u32>,
        }

        let com_con = COMLibrary::new()?;
        let wmi_con = WMIConnection::new(com_con)?;

        // Very broad query, in production this needs strict escaping
        let query = format!("SELECT Name, Status, ConfigManagerErrorCode FROM Win32_PnPEntity WHERE Name LIKE '%{}%'", device_id);

        let results: Vec<Win32_PnPEntity> = wmi_con.raw_query(&query)?;

        if let Some(device) = results.first() {
            Ok(ObservationData::DeviceState {
                name: device.name.clone().unwrap_or_else(|| device_id.to_string()),
                status: device
                    .status
                    .clone()
                    .unwrap_or_else(|| "Unknown".to_string()),
                error_code: device.config_manager_error_code.map(|c| c as i64),
            })
        } else {
            Ok(ObservationData::DeviceState {
                name: device_id.to_string(),
                status: "Missing".to_string(),
                error_code: None,
            })
        }
    }

    fn observe_service(&self, service_name: &str) -> Result<ObservationData> {
        use serde::Deserialize;
        use wmi::{COMLibrary, WMIConnection};

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "PascalCase")]
        struct Win32_Service {
            name: String,
            state: String,
        }

        let com_con = COMLibrary::new()?;
        let wmi_con = WMIConnection::new(com_con)?;

        let query = format!(
            "SELECT Name, State FROM Win32_Service WHERE Name = '{}'",
            service_name
        );
        let results: Vec<Win32_Service> = wmi_con.raw_query(&query)?;

        if let Some(service) = results.first() {
            Ok(ObservationData::ServiceState {
                name: service.name.clone(),
                status: service.state.to_lowercase(),
            })
        } else {
            Err(anyhow!("Service '{}' not found via WMI", service_name))
        }
    }

    fn observe_event_log(
        &self,
        source: &str,
        time_window_hours: u32,
    ) -> Result<Vec<ObservationData>> {
        // Querying Windows Event Log via PowerShell as a reliable cross-version mechanism
        use std::process::Command;

        let script = format!(
            "Get-WinEvent -FilterHashtable @{{ProviderName='{}'; StartTime=(Get-Date).AddHours(-{})}} -MaxEvents 10 | Select-Object Id | ConvertTo-Json",
            source, time_window_hours
        );

        let output = Command::new("powershell")
            .args(&["-NoProfile", "-Command", &script])
            .output()?;

        if output.status.success() {
            // Simplified parsing - in reality, parse JSON output to extract Event IDs
            // For now, return a placeholder event if successful
            Ok(vec![ObservationData::EventLog {
                source: source.to_string(),
                event_id: 0,
            }])
        } else {
            Ok(vec![])
        }
    }

    fn observe_wmi(&self, query: &str) -> Result<Vec<ObservationData>> {
        // Generic WMI query endpoint (simplified)
        use std::collections::HashMap;
        use wmi::{COMLibrary, WMIConnection};

        let com_con = COMLibrary::new()?;
        let wmi_con = WMIConnection::new(com_con)?;

        // Generic raw query returning HashMaps
        let _results: Vec<HashMap<String, wmi::Variant>> = wmi_con.raw_query(query)?;

        // For demonstration, map generic success to an empty list rather than full Variant parsing
        Ok(vec![])
    }
}
