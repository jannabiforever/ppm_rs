use clap::Args;
use ppm_core::services::project::{ListProjects, ProjectFilter};

use crate::commands::CommandHandler;

#[derive(Args, Debug)]
pub struct ListCommand {
	/// Filter projects by status
	#[arg(short, long, value_enum)]
	pub filter: Option<FilterOption>,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum FilterOption {
	All,
	Active,
	Inactive,
}

impl ListCommand {
	pub fn new(filter: Option<FilterOption>) -> Self {
		Self {
			filter,
		}
	}
}

impl CommandHandler for ListCommand {
	fn build_service(
		self,
		context: ppm_core::context::PPMContext,
	) -> Box<dyn ppm_core::services::Service> {
		let filter = self.filter.map(|f| match f {
			FilterOption::All => ProjectFilter::All,
			FilterOption::Active => ProjectFilter::Active,
			FilterOption::Inactive => ProjectFilter::Inactive,
		});

		Box::new(ListProjects {
			project_repository: context.project_repository.clone(),
			output_writer: context.output_writer.clone(),
			filter,
		})
	}
}
