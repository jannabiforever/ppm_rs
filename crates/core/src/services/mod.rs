pub mod end_focus_session;
pub mod start_focus_session;

use crate::errors::PPMResult;

pub trait Service {
	type Output;

	fn run(self) -> PPMResult<Self::Output>;
}
