use clap::{Parser, Subcommand};
use fault_injection::runner::Scenario;

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
                    
                    // Здесь будет запуск теста
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
            for name in Scenario::list_scenarios() {
                println!("  • {}", name);
            }
        }
        Commands::Stats => {
            println!("📊 Test Statistics:");
            println!("  (Coming soon...)");
        }
    }
}

fn run_test(_scenario: &Scenario) {
    println!("🏃 Running test...");
    // TODO: реализовать запуск теста с эмуляцией CAN
}