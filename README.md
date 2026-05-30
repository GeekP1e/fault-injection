# 🔥 Fault Injection Framework (FIF)

[![Rust](https://img.shields.io/badge/rust-1.82+-blue.svg)](https://rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Tests](https://github.com/GeekP1e/fault-injection/actions/workflows/ci.yml/badge.svg)](https://github.com/GeekP1e/fault-injection/actions)

**FIF** — это фреймворк на Rust для тестирования отказоустойчивости систем транспортной безопасности путём внедрения ошибок в эмулируемую CAN-шину.

---

## 🎯 Основные возможности

| Тип отказа | Описание | Параметры | Реальный аналог |
|------------|----------|-----------|-----------------|
| **BitFlip** | Инверсия случайного бита в сообщении | `probability` (0.0–1.0) | Электромагнитные помехи, радиопомехи |
| **MessageLoss** | Потеря сообщения | `rate` (0.0–1.0) | Обрыв провода, коллизия в шине |
| **BusOff** | Отключение устройства от шины на время | `duration_ms` | Обрыв кабеля, ошибка контроллера |
| **Delay** | Задержка доставки сообщения | `duration_ms` | Перегруженная сеть, проблемы с процессором |

---

## 🚀 Быстрый старт

### Установка

```bash
git clone https://github.com/GeekP1e/fault-injection.git
cd fault-injection
cargo build --release
```

### Запуск тестов

```bash
# Все тесты
cargo test

# Только интеграционные тесты
cargo test --test integration

# Конкретный тест
cargo test test_bus_off_recovery
```

### Использование CLI

```bash
# Список доступных сценариев
cargo run -- list

# Запуск сценария
cargo run -- run --scenario critical.yaml

# Сокращённая форма
cargo run -- run -s critical.yaml
```

### Пример YAML-сценария

```yaml
name: "Critical Brake System Test"
description: "Тест тормозной системы при экстремальных отказах"
duration_ms: 30000

faults:
  - type: BitFlip
    probability: 0.05
    start_ms: 0
    
  - type: MessageLoss
    rate: 0.02
    start_ms: 5000
    end_ms: 25000
    
  - type: BusOff
    duration_ms: 100
    start_ms: 10000
    
  - type: Delay
    duration_ms: 50
    start_ms: 15000
    end_ms: 20000

monitors:
  - metric: "message_count"
    expected_min: 1000
  - metric: "error_rate"
    expected_max: 0.10
```

### Использование как библиотеки

```rust
use fault_injection::{CanEmulator, CanMessage, FaultType};

fn main() -> Result<(), String> {
    let mut emulator = CanEmulator::new();
    
    // Внедряем отказы
    emulator.inject_fault(FaultType::BitFlip { probability: 0.1 });
    emulator.inject_fault(FaultType::Delay { duration_ms: 20 });
    emulator.inject_fault(FaultType::MessageLoss { rate: 0.05 });
    
    // Создаём и отправляем сообщение
    let mut msg = CanMessage::new(0x123, [1, 2, 3, 4, 5, 6, 7, 8], 1000);
    
    match emulator.send(&mut msg) {
        Ok(()) => println!("✅ Сообщение доставлено: {:?}", msg.data),
        Err(e) => println!("❌ Ошибка: {}", e),
    }
    
    Ok(())
}
```

## 📁 Структура проекта

```text
fault-injection/
├── src/
│   ├── bin/
│   │   └── fiu.rs              # CLI точка входа
│   ├── can/
│   │   ├── emulator.rs         # CAN эмулятор
│   │   └── message.rs          # Структура CAN сообщения
│   ├── fault/
│   │   └── types.rs            # Типы отказов (enum)
│   ├── logger/
│   │   └── sqlite.rs           # SQLite логгер
│   └── runner/
│       └── scenario.rs         # Загрузка YAML сценариев
├── tests/
│   └── integration.rs          # Интеграционные тесты
├── scenarios/
│   ├── critical.yaml
│   └── network_issues.yaml
└── Cargo.toml
```
## ⚠️ Дисклеймер

Проект создан в образовательных целях для тестирования систем в контролируемой среде. Не используйте для атак на реальные системы.

## 📧 Контакты

[![GitHub](https://img.shields.io/badge/GitHub-GeekP1e-181717?logo=github)](https://github.com/GeekP1e)
[![Проект](https://img.shields.io/badge/Проект-fault--injection-blue)](https://github.com/GeekP1e/fault-injection)