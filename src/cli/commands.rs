// https://docs.rs/clap/latest/clap/index.html

use clap::{command, value_parser, arg, ArgMatches};

pub struct CLI {
    args: ArgMatches,
}

pub enum Command {
    ClearCache,
    StartServer(u16, String)
}

impl CLI {
    pub fn init() -> Self {
        let matches = command!()
            .arg_required_else_help(true)
            .about("Deez Caching Proxy..")
            .subcommand(command!("clear-cache").about("Clear the cache entirely"))
            .arg(arg!(--port [PORT] "Sets the port the server should start on").default_value("3000").requires("origin").value_parser(value_parser!(u16)))
            .arg(arg!(--origin <ORIGIN> "Sets the origin endpoint to proxy").value_parser(value_parser!(String)))
            .get_matches();

        Self { args: matches }
    }

    pub fn command(&self) -> Command {
        match self.args.subcommand() {
            Some(("clear-cache", _s)) => Command::ClearCache,
            _ => {
                let port = self.args.get_one::<u16>("port").unwrap();
                let origin = self.args.get_one::<String>("origin").unwrap().clone();

                Command::StartServer(*port, origin)
            }
        }
    }
}