# GUIDE.md

# Testing Guide for Rust CLI Applications

This guide defines the recommended testing structure and workflow for console/CLI applications developed in Rust.

---

# Recommended Structure

```txt
project/
├── src/
│   ├── main.rs
│   └── ...
├── tests/
│   ├── commands/
│   ├── integration/
│   ├── platform/
│   │   ├── windows/
│   │   └── linux/
│   ├── fixtures/
│   └── mocks/
│       └── fake_api.rs
└── Cargo.toml
```

---

# Test Types

## 1. Unit Tests

Unit tests should be placed close to the source code they test.

Example:

```rust
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum(2, 3), 5);
    }
}
```

Run:

```bash
cargo test
```

---

## 2. Integration Tests

Integration tests validate the application as a real executable.

Example:

```rust
use std::process::Command;

#[test]
fn test_help() {
    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("Usage"));
}
```

---

# Platform-Specific Tests

Only create platform-specific folders if behavior differs between operating systems.

Examples:

- PATH manipulation
- Registry access
- Permissions
- Symlink behavior
- File system specifics

## Windows-only test

```rust
#[cfg(target_os = "windows")]
#[test]
fn test_windows_registry() {
}
```

## Linux-only test

```rust
#[cfg(target_os = "linux")]
#[test]
fn test_linux_permissions() {
}
```

---

# Recommended Dependencies

Add the following to `Cargo.toml`:

```toml
[dev-dependencies]
assert_cmd = "2"
predicates = "3"
tempfile = "3"
serial_test = "3"
```

---

# Useful Libraries

## assert_cmd

Used for testing CLI commands.

Example:

```rust
use assert_cmd::Command;

#[test]
fn test_version() {
    let mut cmd = Command::cargo_bin("pvm").unwrap();

    cmd.arg("--version")
       .assert()
       .success();
}
```

---

## tempfile

Creates isolated temporary environments for tests.

Example:

```rust
use tempfile::tempdir;

#[test]
fn test_temp_environment() {
    let dir = tempdir().unwrap();

    assert!(dir.path().exists());
}
```

---

## serial_test

Useful when tests modify:

- environment variables
- PATH
- symlinks
- global state

Example:

```rust
use serial_test::serial;

#[test]
#[serial]
fn test_path_changes() {
}
```

---

# Fixtures

Fixtures are static files used during testing.

Example structure:

```txt
tests/
└── fixtures/
    ├── fake_python/
    ├── invalid_config/
    └── sample_env/
```

---

# Mocks

Mocks simulate external services or APIs.

Example:

```rust
pub struct FakeApi;

impl FakeApi {
    pub fn get_versions() -> Vec<&'static str> {
        vec!["3.11.0", "3.12.0"]
    }
}
```

---

# Running Tests

Run all tests:

```bash
cargo test
```

Run a specific test:

```bash
cargo test install
```

Show console output:

```bash
cargo test -- --nocapture
```

Run tests sequentially:

```bash
cargo test -- --test-threads=1
```

---

# CI/CD Multi-Platform Testing

Recommended:

- Windows
- Linux
- macOS

Example GitHub Actions matrix:

```yaml
strategy:
  matrix:
    os: [windows-latest, ubuntu-latest, macos-latest]
```

---

# Best Practices

- Keep tests isolated.
- Avoid modifying the real system.
- Use temporary directories whenever possible.
- Prefer integration tests for CLI behavior.
- Keep fixtures minimal and reproducible.
- Separate platform-specific logic.
- Test error cases, not only success cases.
- Ensure commands return correct exit codes.

---

# Suggested Workflow

1. Implement feature
2. Add unit tests
3. Add CLI integration tests
4. Add platform-specific tests if needed
5. Run:
   ```bash
   cargo test
   ```
6. Validate on Windows/Linux/macOS
7. Merge

---

# Recommended Testing Philosophy for CLI Apps

For console applications:

- prioritize real command execution tests
- validate stdout/stderr
- validate exit codes
- validate filesystem effects
- validate environment handling

The closer the test is to real user behavior, the more valuable it becomes.
