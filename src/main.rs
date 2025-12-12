mod commands;
mod errors;

use clap::{Parser, Subcommand};
use commands::CommandHandler;
use ppm_core::config::Config;
use ppm_core::context::PPMContext;
use ppm_core::services::Service;

#[derive(Parser, Debug)]
#[command(name = "ppm")]
#[command(about = "Project Protocol Manager (CLI)", long_about = None)]
pub struct PPMCli {
	#[command(subcommand)]
	pub command: PPMCommand,
}

#[derive(Subcommand, Debug)]
pub enum PPMCommand {
	Start(commands::start::StartCommand),
	End(commands::end::EndCommand),
}

/// Entry point: Load config → Assemble dependencies → Execute command
///
/// Three-step pattern:
/// 1. Create Command from CLI args
/// 2. Build Service from Command (injecting dependencies via context)
/// 3. Execute Service to perform business logic
fn main() -> Result<(), errors::PPMCliError> {
	// Load configuration
	let config = Config::load()?;
	config.validate()?;

	// Assemble dependencies (Dependency Injection)
	let context = PPMContext::new(config);

	// Parse CLI arguments
	let cli = PPMCli::parse();

	// Build and execute service
	match cli.command {
		PPMCommand::Start(command) => {
			command.build_service(context).run()?;
		}
		PPMCommand::End(command) => {
			command.build_service(context).run()?;
		}
	}

	Ok(())
}
