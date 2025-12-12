use std::sync::Arc;

use chrono::Utc;
use ppm_core::clock::{Clock, FixedClock};
use ppm_core::config::Config;
use ppm_core::context::PPMContext;
use ppm_core::output::InMemoryWriter;
use ppm_core::repositories::InMemorySessionRepository;
use ppm_core::services::Service;
use ppm_rs::commands::CommandHandler;
use ppm_rs::commands::session::cancel::CancelCommand;
use ppm_rs::commands::session::end::EndCommand;
use ppm_rs::commands::session::list::ListCommand;
use ppm_rs::commands::session::start::StartCommand;
use ppm_rs::commands::session::stats::StatsCommand;
use ppm_rs::commands::session::status::StatusCommand;

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
	assert_eq!(output[1], "[ppm] Duration: 30 minutes");
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
	assert_eq!(output[1], "[ppm] Duration: 60 minutes"); // default
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
	assert!(output[1].starts_with("[ppm] Session ID: session_"));
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
	assert_eq!(start_output[1], "[ppm] Duration: 45 minutes");

	writer.clear();

	// Act & Assert: End session
	let end_command = EndCommand {};
	let end_service = end_command.build_service(context);
	assert!(end_service.run().is_ok());

	let end_output = writer.get_output();
	assert_eq!(end_output[0], "[ppm] Focus session ended");
	assert!(end_output[1].starts_with("[ppm] Session ID:"));
}

#[test]
fn test_status_command_shows_no_session() {
	// Arrange
	let (context, _clock, writer) = create_test_context();
	let command = StatusCommand::new();

	// Act
	let service = command.build_service(context);
	let result = service.run();

	// Assert
	assert!(result.is_ok());

	let output = writer.get_output();
	assert_eq!(output.len(), 1);
	assert_eq!(output[0], "[ppm] No active focus session");
}

#[test]
fn test_stats_command_shows_no_sessions() {
	// Arrange
	let (context, _clock, writer) = create_test_context();
	let command = StatsCommand::new();

	// Act
	let service = command.build_service(context);
	let result = service.run();

	// Assert
	assert!(result.is_ok());

	let output = writer.get_output();
	assert_eq!(output.len(), 1);
	assert_eq!(output[0], "[ppm] No focus sessions found");
}

#[test]
fn test_stats_command_calculates_statistics() {
	// Arrange
	let (context, clock, writer) = create_test_context();

	// Create 3 sessions with different durations
	let durations = [30, 45, 60];
	for (i, &duration) in durations.iter().enumerate() {
		let start = StartCommand {
			duration: Some(duration),
		};
		start.build_service(context.clone()).run().expect("Start should succeed");

		// Advance time by session duration
		clock.advance(chrono::Duration::minutes(duration as i64));

		let end = EndCommand {};
		end.build_service(context.clone()).run().expect("End should succeed");

		// Advance time for next session
		if i < durations.len() - 1 {
			clock.advance(chrono::Duration::minutes(1));
		}
	}

	writer.clear();

	// Act
	let stats_command = StatsCommand::new();
	let stats_service = stats_command.build_service(context);
	let result = stats_service.run();

	// Assert
	assert!(result.is_ok());

	let output = writer.get_output();
	assert!(output.len() >= 5);
	assert_eq!(output[0], "[ppm] Focus Session Statistics");
	assert_eq!(output[1], "[ppm] ");
	assert!(output[2].contains("Today:"));
	assert!(output[2].contains("3 sessions"));
	assert!(output[3].contains("This week:"));
	assert!(output[4].contains("All time:"));
	assert!(output[4].contains("3 sessions"));
}

#[test]
fn test_stats_command_formats_duration_correctly() {
	// Arrange
	let (context, clock, writer) = create_test_context();

	// Create a session with 90 minutes (1h 30m)
	let start = StartCommand {
		duration: Some(90),
	};
	start.build_service(context.clone()).run().expect("Start should succeed");

	// Advance time by 90 minutes to simulate session duration
	clock.advance(chrono::Duration::minutes(90));

	let end = EndCommand {};
	end.build_service(context.clone()).run().expect("End should succeed");

	writer.clear();

	// Act
	let stats_command = StatsCommand::new();
	let stats_service = stats_command.build_service(context);
	let result = stats_service.run();

	// Assert
	assert!(result.is_ok());

	let output = writer.get_output();
	// Average of one 90-minute session should be 1h 30m
	// Check in the "Average session:" line
	let avg_line = output.iter().find(|line| line.contains("Average session:"));
	assert!(avg_line.is_some(), "No average session line found. Output: {:?}", output);
	let avg_text = avg_line.unwrap();
	assert!(
		avg_text.contains("1h 30m"),
		"Expected '1h 30m' in '{}'. Full output: {:?}",
		avg_text,
		output
	);
}

#[test]
fn test_list_command_shows_no_sessions() {
	// Arrange
	let (context, _clock, writer) = create_test_context();
	let command = ListCommand::new(None);

	// Act
	let service = command.build_service(context);
	let result = service.run();

	// Assert
	assert!(result.is_ok());

	let output = writer.get_output();
	assert_eq!(output.len(), 1);
	assert_eq!(output[0], "[ppm] No focus sessions found");
}

#[test]
fn test_list_command_shows_sessions() {
	// Arrange
	let (context, clock, writer) = create_test_context();

	// Create multiple sessions
	let start1 = StartCommand {
		duration: Some(30),
	};
	start1.build_service(context.clone()).run().expect("Start 1 should succeed");

	let end1 = EndCommand {};
	end1.build_service(context.clone()).run().expect("End 1 should succeed");

	// Advance time so we can start a new session
	clock.advance(chrono::Duration::minutes(31));

	let start2 = StartCommand {
		duration: Some(45),
	};
	start2.build_service(context.clone()).run().expect("Start 2 should succeed");

	writer.clear();

	// Act
	let list_command = ListCommand::new(None);
	let list_service = list_command.build_service(context);
	let result = list_service.run();

	// Assert
	assert!(result.is_ok());

	let output = writer.get_output();
	assert!(output.len() > 2);
	assert_eq!(output[0], "[ppm] Focus sessions (2)");
	assert_eq!(output[1], "[ppm] ");
}

#[test]
fn test_list_command_respects_limit() {
	// Arrange
	let (context, clock, writer) = create_test_context();

	// Create 3 sessions
	for i in 0..3 {
		let start = StartCommand {
			duration: Some(30),
		};
		start.build_service(context.clone()).run().expect("Start should succeed");

		let end = EndCommand {};
		end.build_service(context.clone()).run().expect("End should succeed");

		// Advance time so we can start another session
		if i < 2 {
			clock.advance(chrono::Duration::minutes(31));
		}
	}

	writer.clear();

	// Act
	let list_command = ListCommand::new(Some(2));
	let list_service = list_command.build_service(context);
	let result = list_service.run();

	// Assert
	assert!(result.is_ok());

	let output = writer.get_output();
	assert_eq!(output[0], "[ppm] Focus sessions (2)");
}

#[test]
fn test_status_command_shows_active_session() {
	// Arrange
	let (context, _clock, writer) = create_test_context();

	// Start a session
	let start_command = StartCommand {
		duration: Some(30),
	};
	let start_service = start_command.build_service(context.clone());
	start_service.run().expect("Starting session should succeed");

	writer.clear();

	// Act
	let status_command = StatusCommand::new();
	let status_service = status_command.build_service(context);
	let result = status_service.run();

	// Assert
	assert!(result.is_ok());

	let output = writer.get_output();
	assert_eq!(output.len(), 3);
	assert_eq!(output[0], "[ppm] Focus session active (30 minutes remaining)");
	assert!(output[1].starts_with("[ppm] Started:"));
	assert!(output[2].starts_with("[ppm] Ends:"));
}

#[test]
fn test_cancel_command_cancels_active_session() {
	// Arrange
	let (context, _clock, writer) = create_test_context();

	// Start a session first
	let start_command = StartCommand {
		duration: Some(30),
	};
	let start_service = start_command.build_service(context.clone());
	start_service.run().expect("Starting session should succeed");

	writer.clear();

	// Act
	let cancel_command = CancelCommand::new();
	let cancel_service = cancel_command.build_service(context);
	let result = cancel_service.run();

	// Assert
	assert!(result.is_ok());

	let output = writer.get_output();
	assert_eq!(output.len(), 2);
	assert_eq!(output[0], "[ppm] Focus session cancelled");
	assert!(output[1].starts_with("[ppm] Session ID: session_"));
}

#[test]
fn test_cancel_command_fails_when_no_active_session() {
	// Arrange
	let (context, _clock, _writer) = create_test_context();
	let command = CancelCommand::new();

	// Act
	let service = command.build_service(context);
	let result = service.run();

	// Assert
	assert!(result.is_err());
	let err = result.unwrap_err();
	assert_eq!(err.to_string(), "No active focus session found");
}

#[test]
fn test_cancel_removes_session_completely() {
	// Arrange
	let (context, _clock, writer) = create_test_context();

	// Start a session
	let start_command = StartCommand {
		duration: Some(30),
	};
	let start_service = start_command.build_service(context.clone());
	start_service.run().expect("Starting session should succeed");

	// Cancel the session
	let cancel_command = CancelCommand::new();
	let cancel_service = cancel_command.build_service(context.clone());
	cancel_service.run().expect("Cancelling session should succeed");

	writer.clear();

	// Act - check status should show no session
	let status_command = StatusCommand::new();
	let status_service = status_command.build_service(context);
	status_service.run().expect("Status should succeed");

	// Assert
	let output = writer.get_output();
	assert_eq!(output.len(), 1);
	assert_eq!(output[0], "[ppm] No active focus session");
}
