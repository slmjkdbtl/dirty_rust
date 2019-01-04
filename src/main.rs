// wengwengweng

use clap::{App, AppSettings, Arg, SubCommand};

fn main() {

	let run =
		SubCommand::with_name("run")
			.about("run lua file")
			.arg(Arg::with_name("FILE")
				.takes_value(true)
				.help("the file to run"));

	let export =
		SubCommand::with_name("export")
			.about("cross platform export")
			.arg(Arg::with_name("DIR")
				.takes_value(true)
				.long("dir")
				.help("export dir"));

	let matches = App::new(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.setting(AppSettings::ColoredHelp)
		.setting(AppSettings::VersionlessSubcommands)
		.setting(AppSettings::SubcommandRequiredElseHelp)
		.subcommand(run)
		.subcommand(export)
		.get_matches();

}

