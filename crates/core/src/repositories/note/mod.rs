pub mod in_memory;
pub mod repository;

pub use in_memory::InMemoryNoteRepository;
pub use repository::{LocalNoteRepository, NoteRepository};
