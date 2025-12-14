use clap::Subcommand;

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
