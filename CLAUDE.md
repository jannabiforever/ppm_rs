# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

PPM (Project Protocol Manager) is a Rust-based CLI productivity tool for managing focus sessions. The project follows a clean architecture with strict separation of concerns and dependency injection.

## Build & Development Commands

```bash
# Build the project
cargo build

# Run the application
cargo run -- start -d 30        # Start 30-minute focus session
cargo run -- end                # End active session

# Run all tests (including E2E)
cargo test

# Run specific test file
cargo test --test e2e_tests

# Run specific test function
cargo test test_start_command_creates_session

# Format code (uses hard tabs, see .rustfmt.toml)
cargo fmt

# Check without building
cargo check
```

## Architecture Principles

### Layered Architecture

```
CLI Layer (main.rs)
    ↓ parses arguments
Command Layer (commands/)
    ↓ builds service with dependencies (Factory pattern)
Service Layer (services/)
    ↓ executes business logic
Repository Layer (repositories/)
    ↓ data access
```

**Critical Rule**: Each layer has one responsibility. Commands build services, services execute logic, repositories handle data.

### Dependency Injection

All dependencies are assembled in `main.rs` and passed through `PPMContext`:

```rust
pub struct PPMContext {
    pub config: Config,
    pub clock: Arc<dyn Clock>,
    pub session_repository: Arc<dyn SessionRepository>,
    pub output_writer: Arc<dyn OutputWriter>,
}
```

The context is cloneable (cheap Arc clones) and passed to command handlers.

### Command → Service Pattern

```rust
pub trait CommandHandler {
    type Service: Service<Output = ()>;
    fn build_service(self, context: PPMContext) -> Self::Service;
}
```

**Key Pattern**: Commands don't execute logic—they only create services from context. The service execution happens in `main.rs`.

## Critical Design Rules

### 1. Service Implementation

**DO**: Use direct struct initialization with public fields
```rust
pub struct StartFocusSession {
    pub clock: Arc<dyn Clock>,
    pub repository: Arc<dyn SessionRepository>,
    pub output_writer: Arc<dyn OutputWriter>,
    pub duration_in_minutes: u32,
}

// NO constructor needed!
// Direct initialization in CommandHandler:
StartFocusSession {
    clock: context.clock.clone(),
    repository: context.session_repository.clone(),
    output_writer: context.output_writer.clone(),
    duration_in_minutes: 60,
}
```

**DON'T**: Create constructor methods when there's no validation logic
```rust
// ❌ AVOID
impl StartFocusSession {
    pub fn new(clock: Arc<dyn Clock>, ...) -> Self { ... }
}
```

### 2. Service Trait Usage

**DO**: Implement `Service` trait directly on concrete types
```rust
impl Service for StartFocusSession {
    type Output = ();
    fn run(self) -> PPMResult<()> { ... }
}
```

**DON'T**: Create domain-specific service traits with blanket implementations (causes trait conflicts)
```rust
// ❌ AVOID - causes conflicts with multiple service types
pub trait StartFocusSessionService { ... }
impl<T: StartFocusSessionService> Service for T { ... }
```

### 3. Abstraction Requirements

**MUST abstract**: Time, Output, Data Access
```rust
// Time: NEVER use Utc::now() directly
pub trait Clock: Send + Sync {
    fn now(&self) -> DateTime<Utc>;
}

// Output: NEVER use println! directly
pub trait OutputWriter: Send + Sync {
    fn write_line(&self, message: &dyn Display) -> PPMResult<()>;
}

// Interior mutability for OutputWriter using Mutex<W>
impl<W: io::Write + Send> OutputWriter for Mutex<W> { ... }
```

**WHY**: Enables testing with `FixedClock`, `InMemoryWriter`, `InMemorySessionRepository`

### 4. Error Handling

**MUST**: Use `?` operator, never `unwrap()` in production code
```rust
// ✅ Correct
let writer = self.lock().map_err(|_| PPMError::LockError)?;

// ❌ FORBIDDEN in production
let writer = self.lock().unwrap();
```

Use `thiserror` with `#[from]` for automatic error conversion:
```rust
#[derive(Debug, thiserror::Error)]
pub enum PPMError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
```

### 5. Testing Requirements

**MUST**: Write E2E tests in `tests/` directory, not manual testing
```rust
// tests/e2e_tests.rs
#[test]
fn test_start_command_creates_session() {
    let (context, _clock, writer) = create_test_context();
    let command = StartCommand::new(Some(30));
    let service = command.build_service(context);
    
    assert!(service.run().is_ok());
    assert_eq!(writer.get_output()[0], "[ppm] Focus session started");
}
```

**DON'T**: Use `#[cfg(test)]` on test utilities (makes them unavailable to integration tests)
```rust
// ✅ Correct - always exported
pub struct FixedClock { ... }

// ❌ Wrong - unavailable in tests/
#[cfg(test)]
pub struct FixedClock { ... }
```

## Data Storage

Sessions are stored in `~/.config/ppm/sessions.json` as JSON array. The `LocalSessionRepository` handles:
- Directory creation (`~/.config/ppm/`)
- JSON serialization/deserialization
- Session CRUD operations

Repository methods take `current_time: DateTime<Utc>` as parameter instead of depending on Clock directly (dependency inversion).

## Adding New Commands

1. Create service struct in `crates/core/src/services/`:
```rust
pub struct NewService {
    pub clock: Arc<dyn Clock>,
    pub repository: Arc<dyn SessionRepository>,
    // ... dependencies
}

impl Service for NewService {
    type Output = ();
    fn run(self) -> PPMResult<()> { ... }
}
```

2. Create command in `src/commands/`:
```rust
pub struct NewCommand { /* args */ }

impl CommandHandler for NewCommand {
    type Service = NewService;
    
    fn build_service(self, context: PPMContext) -> Self::Service {
        NewService {
            clock: context.clock.clone(),
            repository: context.session_repository.clone(),
            // ... direct initialization
        }
    }
}
```

3. Add to CLI enum in `src/main.rs`:
```rust
pub enum PPMCommand {
    Start { duration: Option<u32> },
    End,
    New { /* args */ },  // Add here
}
```

4. Wire up in `main.rs` match statement:
```rust
PPMCommand::New { /* args */ } => {
    let command = commands::new::NewCommand::new(/* args */);
    let service = command.build_service(context);
    service.run()?;
}
```

5. Write E2E tests in `tests/e2e_tests.rs`

## Anti-Patterns to Avoid

- ❌ Over-engineering with unnecessary trait hierarchies
- ❌ Creating constructors that only copy fields
- ❌ Using `unwrap()` in production code
- ❌ Direct use of `Utc::now()`, `println!()`, or file I/O without abstraction
- ❌ Making repository depend on Clock (pass time as parameter instead)
- ❌ Manual testing instead of E2E tests
- ❌ Using `#[cfg(test)]` on utilities needed by integration tests

## Key Files

- `src/main.rs` - DI assembly, CLI parsing, command orchestration
- `crates/core/src/context.rs` - DI container with builder pattern
- `crates/core/src/services/mod.rs` - Service trait definition
- `crates/core/src/repositories/session_repository.rs` - Data access abstraction
- `tests/e2e_tests.rs` - End-to-end tests with in-memory mocks
