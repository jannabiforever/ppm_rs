use clap::Subcommand;

pub mod add;
pub mod done;
pub mod list;

#[derive(Debug, Subcommand)]
pub enum TaskCommand {
	/// Add a new task
	Add(add::AddCommand),
	/// List tasks
	List(list::ListCommand),
	/// Mark a task as done
	Done(done::DoneCommand),
}
