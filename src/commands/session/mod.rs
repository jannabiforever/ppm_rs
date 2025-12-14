use clap::Subcommand;

use crate::commands::CommandHandler;

pub mod cancel;
pub mod end;
pub mod list;
pub mod start;
pub mod stats;
pub mod status;

#[derive(Debug, Subcommand)]
pub enum SessionCommand {
	/// Start a new focus session. Fails if already in a session.
	Start(start::StartCommand),
	/// End the current focus session. Fails if not in a session.
	End(end::EndCommand),
	/// Show the current focus session status.
	Status(status::StatusCommand),
	/// Cancel the current focus session without saving it.
	Cancel(cancel::CancelCommand),
	/// List all focus sessions.
	List(list::ListCommand),
	/// Show focus session statistics.
	Stats(stats::StatsCommand),
}

impl CommandHandler for SessionCommand {
	fn build_service(
		self,
		context: ppm_core::context::PPMContext,
	) -> Box<dyn ppm_core::services::Service> {
		match self {
			SessionCommand::Start(c) => c.build_service(context),
			SessionCommand::End(c) => c.build_service(context),
			SessionCommand::Status(c) => c.build_service(context),
			SessionCommand::Cancel(c) => c.build_service(context),
			SessionCommand::List(c) => c.build_service(context),
			SessionCommand::Stats(c) => c.build_service(context),
		}
	}
}
