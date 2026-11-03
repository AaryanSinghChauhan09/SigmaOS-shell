//! SigmaOS-shell - SigmaOS Component CLI
//!
//! Command-line interface for SigmaOS-shell component.

use anyhow::Result;
use clap::{Parser, Subcommand};
use log::info;
use shell::{Component, ComponentStatus, Config, shell};

#[derive(Parser)]
#[command(name = "shell")]
#[command(about = "SigmaOS SigmaOS-shell component", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the component
    Init {
        /// Enable debug mode
        #[arg(short, long)]
        debug: bool,
    },
    /// Start the component
    Start,
    /// Stop the component
    Stop,
    /// Get component status
    Status,
    /// Run the component
    Run {
        /// Enable debug mode
        #[arg(short, long)]
        debug: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let config = Config::default();
    let mut component = shell::new(config);

    match cli.command {
        Commands::Init { debug } => {
            if debug {
                println!("Initializing SigmaOS-shell in debug mode...");
            } else {
                println!("Initializing SigmaOS-shell...");
            }
            component.initialize()?;
            println!("SigmaOS-shell initialized successfully");
        }
        Commands::Start => {
            component.initialize()?;
            component.start()?;
            println!("SigmaOS-shell started successfully");
        }
        Commands::Stop => {
            component.stop()?;
            println!("SigmaOS-shell stopped successfully");
        }
        Commands::Status => {
            let status = component.status();
            println!("SigmaOS-shell status: {:?}", status);
        }
        Commands::Run { debug } => {
            if debug {
                println!("Running SigmaOS-shell in debug mode...");
            } else {
                println!("Running SigmaOS-shell...");
            }
            component.initialize()?;
            component.start()?;
            println!("SigmaOS-shell is running. Press Ctrl+C to stop.");
            
            // Keep running
            tokio::runtime::Runtime::new()?.block_on(async {
                tokio::signal::ctrl_c().await.unwrap();
                info!("Received Ctrl+C, shutting down...");
            });
            
            component.stop()?;
            println!("SigmaOS-shell stopped");
        }
    }

    Ok(())
}
