use std::process::{Command, Output};

/// Exercise the installed command's boundary so parsing and exit behavior are covered together.
fn run(args: &[&str]) -> Output {
	Command::new(env!("CARGO_BIN_EXE_rrainn"))
		.args(args)
		.output()
		.expect("the CLI should run")
}

/// Check discoverable capabilities without coupling tests to help layout or prose.
fn assert_help(output: Output) {
	assert!(output.status.success());
	assert!(output.stderr.is_empty());
	let stdout = String::from_utf8(output.stdout).expect("help should be UTF-8");
	for option in ["--help", "-h", "--version", "-V"] {
		assert!(stdout.contains(option), "help should expose {option}");
	}
}

#[test]
fn help_flags_explain_available_options() {
	for flag in ["--help", "-h"] {
		assert_help(run(&[flag]));
	}
}

#[test]
fn no_arguments_shows_help() {
	assert_help(run(&[]));
}

#[test]
fn version_flags_report_package_version() {
	for flag in ["--version", "-V"] {
		let output = run(&[flag]);
		assert!(output.status.success());
		assert!(output.stderr.is_empty());
		let stdout = String::from_utf8(output.stdout).expect("version should be UTF-8");
		assert_eq!(stdout.trim(), concat!("rrainn ", env!("CARGO_PKG_VERSION")));
	}
}

#[test]
fn invalid_arguments_report_actionable_errors() {
	for argument in ["--unknown", "unexpected"] {
		let output = run(&[argument]);
		assert_eq!(output.status.code(), Some(2));
		assert!(output.stdout.is_empty());
		let stderr = String::from_utf8(output.stderr).expect("errors should be UTF-8");
		assert!(
			stderr.contains(argument),
			"diagnostics should identify the input"
		);
		assert!(
			stderr.contains("--help"),
			"diagnostics should point to help"
		);
	}
}
