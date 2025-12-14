use clap::Args;
use ppm_core::context::PPMContext;
use ppm_core::models::TaskId;
use ppm_core::services::Service;
use ppm_core::services::task::CompleteTask;

use crate::commands::CommandHandler;

#[derive(Args, Debug)]
pub struct DoneCommand {
	/// Task ID to mark as done
	pub task_id: TaskId,
}

impl CommandHandler for DoneCommand {
	fn build_service(self, context: PPMContext) -> Box<dyn Service> {
		Box::new(CompleteTask {
			clock: context.clock.clone(),
			task_repository: context.task_repository.clone(),
			output_writer: context.output_writer.clone(),
			task_id: self.task_id,
		})
	}
}
