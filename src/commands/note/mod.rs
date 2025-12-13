use clap::Subcommand;

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
