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
let writer = self.lock()?;

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

**CLI Layer Error Output**: In `main.rs`, use a separate `run()` function to display user-friendly error messages:
```rust
fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);  // Uses Display from thiserror
        std::process::exit(1);
    }
}

fn run() -> Result<(), PPMError> {
    // Main logic here
}
```

**WHY**: Rust's default `main() -> Result<(), E>` uses `Debug` format (`Error: SessionAlreadyActive`), but we want `Display` format (`Error: A focus session is already active`) from `thiserror`'s `#[error(...)]` attribute.

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

## Domain Models

### Rich Model Pattern with Proc Macros

The project uses procedural macros to create type-safe wrappers around primitive types, eliminating stringly-typed code:

**`#[model_id]`** - Auto-generates ID types with prefix and generation logic:
```rust
use model_macros::model_id;

#[model_id(prefix = "session_", gen = crate::models::gen_id)]
pub struct FocusSessionId(pub String);

// Generated methods:
// - new() -> creates new ID with prefix
// - Display, From<String>, From<&str>, AsRef<str>, Deref<Target=str>
// - Serde transparent serialization
```

**`#[model_name]`** - Creates name wrappers for domain entities:
```rust
use model_macros::model_name;

#[model_name]
pub struct ProjectName(pub String);

// Generated: Display, From, AsRef, Deref, Serde transparent
```

**`#[model]`** - Auto-applies standard derives for domain models:
```rust
use model_macros::model;

#[model]
pub struct FocusSession {
    pub id: FocusSessionId,
    pub start: DateTime<Utc>,
    // ...
}

// Automatically adds: Debug, Clone, Serialize, Deserialize
// Merges with existing derives if present
```

**WHY**: Type safety prevents mixing IDs (can't pass `TaskId` where `FocusSessionId` expected), better API semantics, refactoring safety. The `#[model]` macro ensures consistent derives across all domain models.

**Usage in models**:
```rust
#[model]  // ✅ Use macro for consistent derives
pub struct FocusSession {
    pub id: FocusSessionId,  // ✅ Type-safe
    pub associated_project_name: Option<ProjectName>,  // ✅ Semantic
    // NOT: pub id: String  ❌
    // NOT: #[derive(Debug, Clone, ...)]  ❌ Use #[model] instead
}
```

**Repository signatures use typed IDs**:
```rust
pub trait SessionRepository: Send + Sync {
    fn end_session(&self, session_id: &FocusSessionId, ...) -> PPMResult<()>;
    fn delete_session(&self, session_id: &FocusSessionId) -> PPMResult<()>;
    // NOT: &str ❌
}
```

**Creating instances**:
```rust
// New ID with auto-generation
let id = FocusSessionId::new();  // Generates "session_1234567890"

// From existing string (deserialization, tests)
let id = FocusSessionId::from("session_existing");

// Names from user input
let project = ProjectName::from("my-project");
```

## Data Storage

Sessions are stored in `~/.config/ppm/sessions.json` as JSON array. The `LocalSessionRepository` handles:
- Directory creation (`~/.config/ppm/`)
- JSON serialization/deserialization (IDs serialize transparently as strings)
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

2. Create command in `src/commands/` using clap's `Args` derive:
```rust
use clap::Args;

#[derive(Args, Debug)]
pub struct NewCommand {
    #[arg(short, long)]
    pub some_arg: Option<String>,
}

impl NewCommand {
    pub fn new(some_arg: Option<String>) -> Self {
        Self { some_arg }
    }
}

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

3. Add to CLI enum in `src/main.rs` (clap automatically parses Args):
```rust
#[derive(Subcommand, Debug)]
pub enum PPMCommand {
    Start(commands::start::StartCommand),
    End(commands::end::EndCommand),
    New(commands::new::NewCommand),  // Add here
}
```

4. Wire up in `main.rs` match statement (simplified - command is already parsed):
```rust
PPMCommand::New(command) => {
    command.build_service(context).run()?;
}
```

5. Write E2E tests in `tests/e2e_tests.rs`

**Note**: Commands use `#[derive(Args, Debug)]` from clap. The `new()` constructor is kept for E2E test compatibility, but clap automatically instantiates commands from CLI arguments.

## Anti-Patterns to Avoid

- ❌ Over-engineering with unnecessary trait hierarchies
- ❌ Creating constructors that only copy fields
- ❌ Using `unwrap()` in production code
- ❌ Direct use of `Utc::now()`, `println!()`, or file I/O without abstraction
- ❌ Making repository depend on Clock (pass time as parameter instead)
- ❌ Manual testing instead of E2E tests
- ❌ Using `#[cfg(test)]` on utilities needed by integration tests
- ❌ Using primitive types (`String`, `&str`) for IDs or domain names (use `#[model_id]` or `#[model_name]` instead)

## Key Files

- `src/main.rs` - DI assembly, CLI parsing, command orchestration
- `crates/core/src/context.rs` - DI container with builder pattern
- `crates/core/src/services/mod.rs` - Service trait definition
- `crates/core/src/repositories/session_repository.rs` - Data access abstraction
- `crates/core/src/models/mod.rs` - Domain models with typed IDs and names
- `crates/model_macros/` - Procedural macros for `#[model_id]` and `#[model_name]`
- `tests/e2e_tests.rs` - End-to-end tests with in-memory mocks
