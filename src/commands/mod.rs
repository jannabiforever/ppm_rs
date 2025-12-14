mod note;
mod project;
mod session;
mod task;

use clap::{Subcommand, command};
use ppm_core::context::PPMContext;
use ppm_core::services::Service;

#[derive(Subcommand, Debug)]
pub enum PPMCommand {
	/// Utilities for focus sessions
	#[command(subcommand)]
	Sess(session::SessionCommand),

	/// Task management
	#[command(subcommand)]
	Task(task::TaskCommand),

	/// Note management
	#[command(subcommand)]
	Note(note::NoteCommand),

	/// Project management
	#[command(subcommand)]
	Project(project::ProjectCommand),
}

impl CommandHandler for PPMCommand {
	fn build_service(self, context: PPMContext) -> Box<dyn Service> {
		match self {
			Self::Sess(c) => c.build_service(context),
			Self::Task(c) => c.build_service(context),
			Self::Note(c) => c.build_service(context),
			Self::Project(c) => c.build_service(context),
		}
	}
}

/// Command layer pattern - builds services from context.
///
/// Commands act as factories: they receive PPMContext and construct
/// the appropriate Service with dependencies. Execution happens in main.rs.
pub trait CommandHandler {
	fn build_service(self, context: PPMContext) -> Box<dyn Service>;
}
