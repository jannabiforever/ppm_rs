use clap::Args;
use ppm_core::context::PPMContext;
use ppm_core::services::task::{ListTasks, TaskFilter};

use crate::commands::CommandHandler;

#[derive(Args, Debug)]
pub struct ListCommand {
	/// Show only pending tasks
	#[arg(long)]
	pub pending: bool,

	/// Show only completed tasks
	#[arg(long)]
	pub done: bool,

	/// Show only canceled tasks
	#[arg(long)]
	pub canceled: bool,
}

impl CommandHandler for ListCommand {
	type Service = ListTasks;

	fn build_service(self, context: PPMContext) -> Self::Service {
		let filter = if self.pending {
			Some(TaskFilter::Pending)
		} else if self.done {
			Some(TaskFilter::Done)
		} else if self.canceled {
			Some(TaskFilter::Canceled)
		} else {
			Some(TaskFilter::All)
		};

		ListTasks {
			task_repository: context.task_repository.clone(),
			output_writer: context.output_writer.clone(),
			filter,
		}
	}
}
