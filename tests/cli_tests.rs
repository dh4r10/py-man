use assert_cmd::Command;
use predicates::prelude::*;

fn pvm() -> Command {
    Command::cargo_bin("pvm").unwrap()
}

// --- Basic CLI ---

#[test]
fn help_exits_success() {
    pvm().arg("--help").assert().success();
}

#[test]
fn help_contains_binary_name() {
    pvm()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("pvm"));
}

#[test]
fn version_flag_exits_success() {
    pvm().arg("--version").assert().success();
}

#[test]
fn version_flag_shows_semver() {
    pvm()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"\d+\.\d+\.\d+").unwrap());
}

// --- list ---

#[test]
fn list_exits_success_with_no_versions() {
    pvm().arg("list").assert().success();
}

// --- env ---

#[test]
fn env_bash_outputs_export_path() {
    pvm()
        .args(["env", "--shell", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("export PATH="))
        .stdout(predicate::str::contains(".pvm/bin"));
}

#[test]
fn env_zsh_outputs_export_path() {
    pvm()
        .args(["env", "--shell", "zsh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("export PATH="));
}

#[test]
fn env_fish_outputs_set_gx() {
    pvm()
        .args(["env", "--shell", "fish"])
        .assert()
        .success()
        .stdout(predicate::str::contains("set -gx PATH"));
}

#[test]
fn env_powershell_outputs_env_path() {
    pvm()
        .args(["env", "--shell", "power-shell"])
        .assert()
        .success()
        .stdout(predicate::str::contains("$env:PATH"));
}

#[test]
fn env_cmd_outputs_set() {
    pvm()
        .args(["env", "--shell", "cmd"])
        .assert()
        .success()
        .stdout(predicate::str::contains("@SET"));
}

// --- version validation ---

#[test]
fn use_invalid_version_fails() {
    pvm().args(["use", "invalid"]).assert().failure();
}

#[test]
fn use_letters_in_version_fails() {
    pvm().args(["use", "3.12.x"]).assert().failure();
}

#[test]
fn install_invalid_version_fails() {
    pvm().args(["install", "not-a-version"]).assert().failure();
}

#[test]
fn uninstall_invalid_version_fails() {
    pvm().args(["uninstall", "bad"]).assert().failure();
}

#[test]
fn default_invalid_version_fails() {
    pvm().args(["default", "bad"]).assert().failure();
}

// --- unknown subcommand ---

#[test]
fn unknown_subcommand_fails() {
    pvm().arg("foobar").assert().failure();
}

// --- Platform-specific env auto-detection ---

#[cfg(target_os = "windows")]
#[test]
fn env_auto_detects_powershell_on_windows() {
    pvm()
        .arg("env")
        .assert()
        .success()
        .stdout(predicate::str::contains("$env:PATH"));
}

#[cfg(target_os = "linux")]
#[test]
fn env_auto_detects_bash_on_linux() {
    pvm()
        .env("SHELL", "/bin/bash")
        .env_remove("PSModulePath")
        .arg("env")
        .assert()
        .success()
        .stdout(predicate::str::contains("export PATH="));
}
