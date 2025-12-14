use clap::Subcommand;

use crate::commands::CommandHandler;

pub mod done;
pub mod list;
pub mod new;

#[derive(Debug, Subcommand)]
pub enum TaskCommand {
	/// Add a new task
	New(new::NewCommand),
	/// List tasks
	List(list::ListCommand),
	/// Mark a task as done
	Done(done::DoneCommand),
}

impl CommandHandler for TaskCommand {
	fn build_service(
		self,
		context: ppm_core::context::PPMContext,
	) -> Box<dyn ppm_core::services::Service> {
		match self {
			TaskCommand::New(c) => c.build_service(context),
			TaskCommand::List(c) => c.build_service(context),
			TaskCommand::Done(c) => c.build_service(context),
		}
	}
}
