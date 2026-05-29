use rand::RngExt;
use crate::can::CanMessage;
use crate::fault::FaultType;

pub struct CanEmulator {
    faults: Vec<FaultType>,
    bus_off_until: Option<u64>,
    active_bus_off: bool,
}

impl CanEmulator {
    pub fn new() -> Self {
        Self {
            faults: Vec::new(),
            bus_off_until: None,
            active_bus_off: false,
        }
    }

    pub fn inject_fault(&mut self, fault: FaultType) {
        self.faults.push(fault);
    }
    
    pub fn clear_faults(&mut self) {
        self.faults.clear();
        self.bus_off_until = None;
        self.active_bus_off = false;
    }

    
    pub fn send(&mut self, msg: &mut CanMessage) -> Result<(), String> {
        let now = msg.timestamp;
        
        self.check_bus_off(now)?;
        
        for i in 0..self.faults.len() {
            if matches!(self.faults[i], FaultType::BusOff { .. }) && self.bus_off_until.is_some() {
                continue;
            }
            
            let fault = self.faults[i].clone();
            self.apply_fault(&fault, msg)?;
        }
        
        Ok(())
    }

    fn check_bus_off(&mut self, now: u64) -> Result<(), String> {
        if let Some(until) = self.bus_off_until {
            if now < until {
                return Err("Bus off triggered".to_string());
            } else {
                self.bus_off_until = None;
                self.active_bus_off = false;
                self.faults.retain(|f| !matches!(f, FaultType::BusOff { .. }));
            }
        }
        Ok(())
    }

    fn apply_fault(&mut self, fault: &FaultType, msg: &mut CanMessage) -> Result<(), String> {
        let now = msg.timestamp;
        let mut rng = rand::rng();
        
        match fault {
            FaultType::BitFlip { probability } => {
                if rng.random_bool(*probability) {
                    let byte = rng.random_range(0..8);
                    let bit = rng.random_range(0..8);
                    msg.data[byte] ^= 1 << bit;
                }
                Ok(())
            }
            FaultType::MessageLoss { rate } => {
                if rng.random_bool(*rate) {
                    Err("Message lost".to_string())
                } else {
                    Ok(())
                }
            }
            FaultType::BusOff { duration_ms } => {
                // Устанавливаем Bus Off только если он еще не активен
                if !self.active_bus_off {
                    self.bus_off_until = Some(now + duration_ms);
                    self.active_bus_off = true;
                    Err("Bus off triggered".to_string())
                } else {
                    Ok(())
                }
            }
            FaultType::Delay { duration_ms } => {
                std::thread::sleep(std::time::Duration::from_millis(*duration_ms));
                Ok(())
            }
        }
    }
}