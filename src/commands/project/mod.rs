use clap::Subcommand;

mod list;
mod new;

use crate::commands::CommandHandler;

#[derive(Subcommand, Debug)]
pub enum ProjectCommand {
	New(new::NewCommand),
	List(list::ListCommand),
}

impl CommandHandler for ProjectCommand {
	fn build_service(
		self,
		context: ppm_core::context::PPMContext,
	) -> Box<dyn ppm_core::services::Service> {
		match self {
			ProjectCommand::New(new_command) => new_command.build_service(context),
			ProjectCommand::List(list_command) => list_command.build_service(context),
		}
	}
}
