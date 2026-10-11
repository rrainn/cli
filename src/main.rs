use clap::Parser;

/// Keep command metadata in Cargo so help and version stay aligned with releases.
#[derive(Parser)]
#[command(version, about)]
struct Cli {}

fn main() {
	// Until commands are added, an empty invocation should guide users to available options.
	if std::env::args_os().len() == 1 {
		Cli::parse_from(["rrainn", "--help"]);
		return;
	}

	Cli::parse();
}
