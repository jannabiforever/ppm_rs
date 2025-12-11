mod errors;

use clap::{Parser, Subcommand};

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
}

fn load_config() -> Result<(), errors::PPMCliError> {
	Ok(())
}

fn setup() -> Result<(), errors::PPMCliError> {
	Ok(())
}

fn main() -> Result<(), errors::PPMCliError> {
	// Setup
	load_config()?;

	setup()?;

	let cli = PPMCli::parse();

	match cli.command {
		PPMCommand::Start {
			duration,
		} => {
			println!("[ppm] Starting focus session");

			if let Some(duration) = duration {
				println!("Duration: {} seconds", duration);
			} else {
				println!("Duration: 60 seconds");
			}
		}
	}

	Ok(())
}
