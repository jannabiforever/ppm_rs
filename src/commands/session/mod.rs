use clap::Subcommand;

pub mod end;
pub mod start;

#[derive(Debug, Subcommand)]
pub enum SessionCommand {
	/// Start a new focus session. Fails if already in a session.
	Start(start::StartCommand),
	/// End the current focus session. Fails if not in a session.
	End(end::EndCommand),
}
