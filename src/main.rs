#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_json;

use clap::{App, Arg, SubCommand};

mod app;
#[allow(dead_code)]
mod manager;
#[allow(dead_code)]
mod settings;

fn main() {
    let matches = App::new("zinit")
        .author("Muhamad Azmy, https://github.com/muhamadazmy")
        .version("0.1")
        .about("A runit replacement")
        .subcommand(
            SubCommand::with_name("init")
                .arg(
                    Arg::with_name("config")
                        .value_name("DIR")
                        .short("c")
                        .long("config")
                        .help("service configurations directory")
                        .default_value("/etc/zinit/"),
                )
                .about("run in init mode, start and maintain configured services"),
        )
        .subcommand(
            SubCommand::with_name("status")
                .arg(
                    Arg::with_name("service")
                        .value_name("DIR")
                        .required(false)
                        .help("show status for this service"),
                )
                .about("show service status or all if service is empty"),
        )
        .get_matches();

    let result = match matches.subcommand() {
        ("init", Some(matches)) => app::init(matches.value_of("config").unwrap()),
        ("status", Some(matches)) => app::status(matches.value_of("service")),
        _ => {
            // TODO: replace with a call to default command
            // this now can be `init` but may be a `status` command
            // would be more appropriate
            println!("try help");
            return;
        }
    };

    match result {
        Ok(_) => return,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    }
}
