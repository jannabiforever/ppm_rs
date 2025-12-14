use clap::Subcommand;

use crate::commands::CommandHandler;

pub mod delete;
pub mod list;
pub mod new;

#[derive(Debug, Subcommand)]
pub enum NoteCommand {
	/// Add a new note
	New(new::NewCommand),
	/// List notes
	List(list::ListCommand),
	/// Delete a note
	Delete(delete::DeleteCommand),
}

impl CommandHandler for NoteCommand {
	fn build_service(
		self,
		context: ppm_core::context::PPMContext,
	) -> Box<dyn ppm_core::services::Service> {
		match self {
			NoteCommand::New(c) => c.build_service(context),
			NoteCommand::List(c) => c.build_service(context),
			NoteCommand::Delete(c) => c.build_service(context),
		}
	}
}
