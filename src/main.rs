#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use clap::{Arg, App, SubCommand};

fn main()
{
	let app_args = App::new("* Filler Killer *")
		.version(env!("CARGO_PKG_VERSION"))
		.author("Emilian C. https://github.com/EmilianC")
		.about("Downloads and trims filler from YouTube videos.")

		.arg(Arg::with_name("interactive").short("i")
			.help("Use a CLI menu to take actions"))

		.arg(Arg::with_name("notifications").short("n")
			.conflicts_with("clear")
			.help("Show pending notifications"))

		.arg(Arg::with_name("clear").short("c")
			.help("Clear all notifications"))

		.arg(Arg::with_name("update").short("u")
			.takes_value(true)
			.min_values(0).max_values(1)
			.value_name("channel")
			.help("Updates channel content [default: all]"))

		.get_matches();

	if (app_args.is_present("notifications"))
	{
		println!("Showing notifications!");
	}
	else if (app_args.is_present("clear"))
	{
		println!("Clearing notifications!");
	}
	else if (app_args.is_present("interactive"))
	{
		println!("Interactive mode!");
	}
	else if (app_args.is_present("update"))
	{
		match app_args.value_of("update") {
			None          => { println!("Update: all"); }
			Some(channel) => { println!("Update: {}", channel); }
		}
	}
	else
	{
		println!("Missing arguments. Run with '-h' or '--help' for usage info.");
	}
}
