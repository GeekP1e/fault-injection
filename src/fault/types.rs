use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FaultType {
    /// Инверсия бита.
    /// Возможные причины: ээлектромагнитные помехи, радиопомехи, плохой контакт
    BitFlip { probability: f64 },
    /// Потеря сообщения.
    /// Возможные причины: обрыв провода, коллизия в шине, переполнение буфера
    MessageLoss { rate: f64 },
    /// Отключение шины.
    /// Возможные причины: обрыв кабеля, ошибка контроллера, откл. устройства
    BusOff { duration_ms: u64 },
    /// Задержка сообщения.
    /// Возможные причины: перегруженная сеть, проблемы с процессором, длинная очередь
    Delay { duration_ms: u64 },
}

impl FaultType {
    pub fn description(&self) -> String {
        match self {
            FaultType::BitFlip { probability } => 
                format!("BitFlip (p={:.2})", probability),
            FaultType::MessageLoss { rate } => 
                format!("MessageLoss (rate={:.2})", rate),
            FaultType::BusOff { duration_ms } => 
                format!("BusOff ({}ms)", duration_ms),
            FaultType::Delay { duration_ms } => 
                format!("Delay ({}ms)", duration_ms),
        }
    }
}