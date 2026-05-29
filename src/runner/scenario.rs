use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use crate::fault::FaultType;

#[derive(Debug, Serialize, Deserialize)]
pub struct Scenario {
    pub name: String,
    pub description: String,
    pub duration_ms: u64,
    pub faults: Vec<ScenarioFault>,
    pub monitors: Vec<Monitor>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScenarioFault {
    #[serde(rename = "type")]
    pub fault_type: String,
    pub probability: Option<f64>,
    pub rate: Option<f64>,
    pub duration_ms: Option<u64>,
    pub start_ms: Option<u64>,
    pub end_ms: Option<u64>,
}

impl ScenarioFault {
    pub fn to_fault_type(&self) -> Option<FaultType> {
        match self.fault_type.as_str() {
            "BitFlip" => self.probability.map(|p| FaultType::BitFlip { probability: p }),
            "MessageLoss" => self.rate.map(|r| FaultType::MessageLoss { rate: r }),
            "BusOff" => self.duration_ms.map(|d| FaultType::BusOff { duration_ms: d }),
            "Delay" => self.duration_ms.map(|d| FaultType::Delay { duration_ms: d }),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Monitor {
    pub metric: String,
    pub expected_min: Option<f64>,
    pub expected_max: Option<f64>,
}

impl Scenario {
    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let scenario: Scenario = serde_yaml::from_str(&content)?;
        Ok(scenario)
    }
    
    pub fn list_scenarios() -> Vec<String> {
        let mut scenarios = Vec::new();
        if let Ok(entries) = std::fs::read_dir("scenarios") {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    scenarios.push(name.to_string());
                }
            }
        }
        scenarios
    }
}