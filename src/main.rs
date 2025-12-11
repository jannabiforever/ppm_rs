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
	Start {
		#[arg(short, long)]
		duration: Option<u32>,
	},
	End,
}

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
		PPMCommand::Start {
			duration,
		} => {
			let command = commands::start::StartCommand::new(duration);
			let service = command.build_service(context);
			service.run()?;
		}
		PPMCommand::End => {
			let command = commands::end::EndCommand::new();
			let service = command.build_service(context);
			service.run()?;
		}
	}

	Ok(())
}
