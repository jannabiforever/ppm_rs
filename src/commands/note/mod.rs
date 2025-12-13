use clap::Subcommand;

pub mod add;
pub mod delete;
pub mod list;

#[derive(Debug, Subcommand)]
pub enum NoteCommand {
	/// Add a new note
	New(add::AddCommand),
	/// List notes
	List(list::ListCommand),
	/// Delete a note
	Delete(delete::DeleteCommand),
}
