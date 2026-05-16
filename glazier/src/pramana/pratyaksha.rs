use anyhow::Result;
use crate::memory::working::ObservationData;

pub trait PratyakshaProvider {
    fn observe_device(&self, device_id: &str) -> Result<ObservationData>;
    fn observe_service(&self, service_name: &str) -> Result<ObservationData>;
    fn observe_event_log(&self, source: &str, time_window_hours: u32) -> Result<Vec<ObservationData>>;
    fn observe_wmi(&self, query: &str) -> Result<Vec<ObservationData>>;
}

#[cfg(not(target_os = "windows"))]
pub struct SandboxMock;

#[cfg(not(target_os = "windows"))]
impl PratyakshaProvider for SandboxMock {
    fn observe_device(&self, device_id: &str) -> Result<ObservationData> {
        println!("TRACE [PRATYAKSHA MOCK] observing device: {}", device_id);
        // Hardcode a mock state returning error code 43 for microphone as an example.
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
        println!("TRACE [PRATYAKSHA MOCK] observing service: {}", service_name);
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

    fn observe_event_log(&self, source: &str, _time_window_hours: u32) -> Result<Vec<ObservationData>> {
        println!("TRACE [PRATYAKSHA MOCK] observing event log source: {}", source);
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
        // Real Win32 SetupDi API calls would go here
        unimplemented!("Real Windows API not yet implemented")
    }

    fn observe_service(&self, service_name: &str) -> Result<ObservationData> {
         unimplemented!("Real Windows API not yet implemented")
    }

    fn observe_event_log(&self, source: &str, time_window_hours: u32) -> Result<Vec<ObservationData>> {
         unimplemented!("Real Windows API not yet implemented")
    }

    fn observe_wmi(&self, query: &str) -> Result<Vec<ObservationData>> {
         unimplemented!("Real WMI API not yet implemented")
    }
}
