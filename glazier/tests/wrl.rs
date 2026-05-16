use glazier::wrl::parser;

#[test]
fn test_wrl_parsing() {
    let input = r#"
adhikara diagnose_microphone {
  target: device(microphone)

  pratyaksha {
    observe: [
      driver_state(realtek_audio),
      service_state(AudioSrv),
      event_log(System, source: "audiodg"),
      device_manager_status(microphone)
    ]
    via: [device_manager, event_log, wmi]
  }

  vyapti post_update_driver_conflict {
    whenever: error_code(43) AND windows_updated(within: 72h)
    always:   cause(driver_version_incompatibility)
    witnessed: [case_seed_001]
    confidence: established
  }

  anumana primary_diagnosis {
    pratijnha:  microphone_not_functioning
    hetu:       error_code(43)
    udaharana:  post_update_driver_conflict
    upanaya:    current_system_matches_hetu
    nigamana:   attempt(rollback_driver(realtek_audio))
  }
}
"#;
    let result = parser::parse_wrl(input);
    if let Err(e) = &result {
        println!("Error: {}", e);
    }
    assert!(result.is_ok());
}
