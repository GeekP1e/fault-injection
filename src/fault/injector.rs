use super::types::FaultType;

pub struct FaultInjector {
    faults: Vec<FaultType>,
}

impl FaultInjector {
    pub fn new() -> Self {
        Self {
            faults: Vec::new(),
        }
    }
    
    pub fn add_fault(&mut self, fault: FaultType) {
        self.faults.push(fault);
    }
    
    pub fn clear_faults(&mut self) {
        self.faults.clear();
    }
    
    pub fn get_faults(&self) -> &[FaultType] {
        &self.faults
    }
    
    pub fn has_faults(&self) -> bool {
        !self.faults.is_empty()
    }
}