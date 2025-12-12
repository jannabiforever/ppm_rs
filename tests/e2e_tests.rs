use std::sync::Arc;

use chrono::Utc;
use ppm_core::clock::{Clock, FixedClock};
use ppm_core::config::Config;
use ppm_core::context::PPMContext;
use ppm_core::output::InMemoryWriter;
use ppm_core::repositories::InMemorySessionRepository;
use ppm_core::services::Service;
use ppm_rs::commands::CommandHandler;
use ppm_rs::commands::session::end::EndCommand;
use ppm_rs::commands::session::start::StartCommand;

fn create_test_context() -> (PPMContext, Arc<FixedClock>, Arc<InMemoryWriter>) {
	let fixed_time = Utc::now();
	let clock = Arc::new(FixedClock::new(fixed_time));
	let repository = Arc::new(InMemorySessionRepository::new());
	let writer = Arc::new(InMemoryWriter::new());

	let context = PPMContext::builder()
		.config(Config::default())
		.clock(clock.clone() as Arc<dyn Clock>)
		.session_repository(repository)
		.output_writer(writer.clone())
		.build();

	(context, clock, writer)
}

#[test]
fn test_start_command_creates_session() {
	// Arrange
	let (context, _clock, writer) = create_test_context();
	let command = StartCommand {
		duration: Some(30),
	};

	// Act
	let service = command.build_service(context);
	let result = service.run();

	// Assert
	assert!(result.is_ok());

	let output = writer.get_output();
	assert_eq!(output.len(), 2);
	assert_eq!(output[0], "[ppm] Focus session started");
	assert_eq!(output[1], "Duration: 30 minutes");
}

#[test]
fn test_start_command_uses_default_duration() {
	// Arrange
	let (context, _clock, writer) = create_test_context();
	let command = StartCommand {
		duration: None,
	};

	// Act
	let service = command.build_service(context);
	let result = service.run();

	// Assert
	assert!(result.is_ok());

	let output = writer.get_output();
	assert_eq!(output[1], "Duration: 60 minutes"); // default
}

#[test]
fn test_start_command_fails_when_session_already_active() {
	// Arrange
	let (context, _clock, _writer) = create_test_context();

	// Start first session
	let command1 = StartCommand {
		duration: Some(30),
	};
	let service1 = command1.build_service(context.clone());
	service1.run().expect("First session should succeed");

	// Try to start second session
	let command2 = StartCommand {
		duration: Some(30),
	};
	let service2 = command2.build_service(context);

	// Act
	let result = service2.run();

	// Assert
	assert!(result.is_err());
	let err = result.unwrap_err();
	assert_eq!(err.to_string(), "A focus session is already active");
}

#[test]
fn test_end_command_ends_active_session() {
	// Arrange
	let (context, _clock, writer) = create_test_context();

	// Start a session first
	let start_command = StartCommand {
		duration: Some(30),
	};
	let start_service = start_command.build_service(context.clone());
	start_service.run().expect("Starting session should succeed");

	writer.clear(); // Clear start output

	// Act
	let end_command = EndCommand {};
	let end_service = end_command.build_service(context);
	let result = end_service.run();

	// Assert
	assert!(result.is_ok());

	let output = writer.get_output();
	assert_eq!(output.len(), 2);
	assert_eq!(output[0], "[ppm] Focus session ended");
	assert!(output[1].starts_with("Session ID: session_"));
}

#[test]
fn test_end_command_fails_when_no_active_session() {
	// Arrange
	let (context, _clock, _writer) = create_test_context();
	let command = EndCommand {};

	// Act
	let service = command.build_service(context);
	let result = service.run();

	// Assert
	assert!(result.is_err());
	let err = result.unwrap_err();
	assert_eq!(err.to_string(), "No active focus session found");
}

#[test]
fn test_full_session_lifecycle() {
	// Arrange
	let (context, _clock, writer) = create_test_context();

	// Act & Assert: Start session
	let start_command = StartCommand {
		duration: Some(45),
	};
	let start_service = start_command.build_service(context.clone());
	assert!(start_service.run().is_ok());

	let start_output = writer.get_output();
	assert_eq!(start_output[0], "[ppm] Focus session started");
	assert_eq!(start_output[1], "Duration: 45 minutes");

	writer.clear();

	// Act & Assert: End session
	let end_command = EndCommand {};
	let end_service = end_command.build_service(context);
	assert!(end_service.run().is_ok());

	let end_output = writer.get_output();
	assert_eq!(end_output[0], "[ppm] Focus session ended");
	assert!(end_output[1].starts_with("Session ID:"));
}
