use clap::{Parser, Subcommand};
use fault_injection::{CanEmulator, CanMessage};
use fault_injection::runner::Scenario;
use std::time::Instant;

#[derive(Parser)]
#[command(name = "fiu")]
#[command(about = "Fault Injection Framework")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Запустить тест по сценарию
    Run {
        #[arg(short, long)]
        scenario: String,
    },

    /// Список доступных сценариев
    List,

    /// Показать статистику тестов
    Stats,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { scenario } => {
            println!("🔍 Loading scenario: {}", scenario);

            let path = std::path::Path::new("scenarios").join(&scenario);
            match Scenario::load(&path) {
                Ok(scenario) => {
                    println!("📋 Name: {}", scenario.name);
                    println!("📝 Description: {}", scenario.description);
                    println!("⏱️ Duration: {} ms", scenario.duration_ms);
                    println!("💉 Faults: {}", scenario.faults.len());

                    run_test(&scenario);
                }
                Err(e) => {
                    eprintln!("❌ Failed to load scenario: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::List => {
            println!("📋 Available scenarios:");
            if let Ok(entries) = std::fs::read_dir("scenarios") {
                for entry in entries.flatten() {
                    if let Some(name) = entry.file_name().to_str() {
                        println!("  • {}", name);
                    }
                }
            } else {
                println!("  No scenarios found in ./scenarios/");
            }
        }
        Commands::Stats => {
            println!("📊 Test Statistics:");
            println!("  (Coming soon...)");
        }
    }
}

fn run_test(scenario: &Scenario) {
    println!("🏃 Running test...");
    let start_time = Instant::now();

    let mut emulator = CanEmulator::new();

    for fault_config in &scenario.faults {
        if let Some(fault) = fault_config.to_fault_type() {
            println!("  💉 Injecting: {}", fault.description());
            emulator.inject_fault(fault);
        }
    }

    let duration_ms = scenario.duration_ms;
    let step_ms = 10;
    let mut messages_sent = 0;
    let mut messages_lost = 0;
    let mut messages_corrupted = 0;

    for timestamp in (0..duration_ms).step_by(step_ms as usize) {
        let mut msg = CanMessage::new(0x123, [timestamp as u8, 0, 0, 0, 0, 0, 0, 0], timestamp);
        let original_data = msg.data;

        match emulator.send(&mut msg) {
            Ok(()) => {
                messages_sent += 1;
                if msg.data != original_data {
                    messages_corrupted += 1;
                }
            }
            Err(e) => {
                if e.contains("lost") {
                    messages_lost += 1;
                }
                messages_sent += 1;
            }
        }
    }

    let elapsed = start_time.elapsed();

    println!();
    println!("📊 Results:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  Duration:           {} ms", duration_ms);
    println!("  Messages sent:      {}", messages_sent);
    println!("  Messages lost:      {}", messages_lost);
    println!("  Messages corrupted: {}", messages_corrupted);
    println!("  Delivery rate:      {:.1}%", 
        (messages_sent - messages_lost) as f64 / messages_sent as f64 * 100.0);
    println!("  Corruption rate:    {:.1}%",
        messages_corrupted as f64 / messages_sent as f64 * 100.0);
    println!("  Real time elapsed:  {:.2} s", elapsed.as_secs_f64());
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}