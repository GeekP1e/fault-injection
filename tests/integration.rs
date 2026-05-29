use fault_injection::{CanEmulator, CanMessage, FaultType};

#[test]
fn test_bitflip_fault() {
    let mut emulator = CanEmulator::new();
    emulator.inject_fault(FaultType::BitFlip { probability: 1.0 });
    
    let original_msg = CanMessage::new(0x123, [0b00000001, 0, 0, 0, 0, 0, 0, 0], 1000);
    let mut test_msg = original_msg.clone();
    
    let result = emulator.send(&mut test_msg);
    assert!(result.is_ok());
    
    assert_ne!(original_msg.data, test_msg.data);
}

#[test]
fn test_message_loss() {
    let mut emulator = CanEmulator::new();
    emulator.inject_fault(FaultType::MessageLoss { rate: 1.0 });
    
    let mut msg = CanMessage::new(0x123, [1, 2, 3, 4, 5, 6, 7, 8], 1000);
    let result = emulator.send(&mut msg);
    
    assert!(result.is_err());
}

#[test]
fn test_no_fault() {
    let mut emulator = CanEmulator::new();
    
    let original_msg = CanMessage::new(0x123, [1, 2, 3, 4, 5, 6, 7, 8], 1000);
    let mut test_msg = original_msg.clone();
    
    let result = emulator.send(&mut test_msg);
    assert!(result.is_ok());
    assert_eq!(original_msg.data, test_msg.data);
}

#[test]
fn test_bus_off_fault() {
    let mut emulator = CanEmulator::new();
    emulator.inject_fault(FaultType::BusOff { duration_ms: 100 });
    
    let mut msg = CanMessage::new(0x123, [1, 2, 3, 4, 5, 6, 7, 8], 1000);
    
    // Первая попытка — BusOff срабатывает
    let result = emulator.send(&mut msg);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Bus off triggered");
}

#[test]
fn test_bus_off_recovery() {
    let mut emulator = CanEmulator::new();
    emulator.inject_fault(FaultType::BusOff { duration_ms: 50 });
    
    let mut msg1 = CanMessage::new(0x123, [1, 2, 3, 4, 5, 6, 7, 8], 1000);
    let result1 = emulator.send(&mut msg1);
    assert!(result1.is_err());
    assert_eq!(result1.unwrap_err(), "Bus off triggered");
    
    let mut msg2 = CanMessage::new(0x123, [1, 2, 3, 4, 5, 6, 7, 8], 1051);
    let result2 = emulator.send(&mut msg2);
    
    assert!(result2.is_ok(), "Expected ok, got: {:?}", result2.err());
}

#[test]
fn test_delay_fault() {
    use std::time::Instant;
    
    let mut emulator = CanEmulator::new();
    emulator.inject_fault(FaultType::Delay { duration_ms: 100 });
    
    let mut msg = CanMessage::new(0x123, [1, 2, 3, 4, 5, 6, 7, 8], 1000);
    
    let start = Instant::now();
    let result = emulator.send(&mut msg);
    let elapsed = start.elapsed();
    
    assert!(result.is_ok());
    assert!(elapsed.as_millis() >= 100);
}

#[test]
fn test_multiple_faults() {
    let mut emulator = CanEmulator::new();
    emulator.inject_fault(FaultType::BitFlip { probability: 1.0 });
    emulator.inject_fault(FaultType::Delay { duration_ms: 10 });
    
    let original_msg = CanMessage::new(0x123, [0b00000001, 0, 0, 0, 0, 0, 0, 0], 1000);
    let mut test_msg = original_msg.clone();
    
    let result = emulator.send(&mut test_msg);
    assert!(result.is_ok());
    assert_ne!(original_msg.data, test_msg.data);  // BitFlip сработал
}

#[test]
fn test_scenario_loading() {
    use fault_injection::runner::Scenario;
    
    let yaml_content = r#"
name: "Test Scenario"
description: "For unit testing"
duration_ms: 1000
faults:
  - type: BitFlip
    probability: 0.5
monitors: []
"#;
    
    let temp_file = "temp_test_scenario.yaml";
    std::fs::write(temp_file, yaml_content).unwrap();
    
    let scenario = Scenario::load(std::path::Path::new(temp_file));
    
    if scenario.is_err() {
        eprintln!("Error loading scenario: {:?}", scenario.as_ref().err());
    }
    
    assert!(scenario.is_ok());
    
    let scenario = scenario.unwrap();
    assert_eq!(scenario.name, "Test Scenario");
    assert_eq!(scenario.duration_ms, 1000);
    assert_eq!(scenario.faults.len(), 1);
    
    std::fs::remove_file(temp_file).unwrap();
}