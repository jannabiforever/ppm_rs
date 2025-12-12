mod commands;

use clap::{Parser, Subcommand};
use commands::CommandHandler;
use ppm_core::config::Config;
use ppm_core::context::PPMContext;
use ppm_core::errors::PPMError;
use ppm_core::services::Service;

use crate::commands::session::SessionCommand;

#[derive(Parser, Debug)]
#[command(name = "ppm")]
#[command(about = "Project Protocol Manager (CLI)", long_about = None)]
pub struct PPMCli {
	#[command(subcommand)]
	pub command: PPMCommand,
}

#[derive(Subcommand, Debug)]
pub enum PPMCommand {
	/// Utilities for focus sessions
	#[command(subcommand)]
	Sess(commands::session::SessionCommand),
}

/// Entry point: Load config → Assemble dependencies → Execute command
///
/// Three-step pattern:
/// 1. Create Command from CLI args
/// 2. Build Service from Command (injecting dependencies via context)
/// 3. Execute Service to perform business logic
fn main() {
	if let Err(e) = run() {
		eprintln!("Error: {}", e);
		std::process::exit(1);
	}
}

fn run() -> Result<(), PPMError> {
	// Load configuration
	let config = Config::load()?;
	config.validate()?;

	// Assemble dependencies (Dependency Injection)
	let context = PPMContext::new(config);

	// Parse CLI arguments
	let cli = PPMCli::parse();

	// Build and execute service
	match cli.command {
		PPMCommand::Sess(SessionCommand::Start(command)) => command.build_service(context).run(),
		PPMCommand::Sess(SessionCommand::End(command)) => command.build_service(context).run(),
	}?;

	Ok(())
}
