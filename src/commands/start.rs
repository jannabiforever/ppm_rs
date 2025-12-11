use ppm_core::context::PPMContext;
use ppm_core::services::Service;
use ppm_core::services::start_focus_session::LocallyStartFocusSession;

use crate::commands::CommandHandler;
use crate::errors::PPMCliError;

pub struct StartCommand {
	pub duration: Option<u32>,
}

impl StartCommand {
	pub fn new(duration: Option<u32>) -> Self {
		Self {
			duration,
		}
	}
}

impl CommandHandler for StartCommand {
	fn execute(self, context: PPMContext) -> Result<(), PPMCliError> {
		let service = LocallyStartFocusSession::new(
			context.clock.clone(),
			context.session_repository.clone(),
			context.output_writer.clone(),
			self.duration.unwrap_or(context.config.default_focus_duration_in_minutes),
		);

		service.run()?;

		Ok(())
	}
}
