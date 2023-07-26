use std::net::SocketAddr;
use clap::{Arg, ArgAction, Command, value_parser};
use clap::builder::PossibleValue;
use env_logger::Env;
use server::ServerError;

fn main() -> anyhow::Result<()> {
    let matches = Command::new("minecrust")
        .arg(
            Arg::new("addr")
                .env("MINECRUST_ADDR")
                .short('A')
                .long("addr")
                .action(ArgAction::Set)
                .value_name("IP:PORT")
                .default_value("127.0.0.1:25565")
                .value_parser(value_parser!(SocketAddr))
                .help("IP address and port to bind to")
                .global(true)
        )
        .arg(
            Arg::new("log")
                .env("MINECRUST_LOG")
                .short('l')
                .long("log")
                .action(ArgAction::Set)
                .value_name("LEVEL")
                .default_value("info")
                .value_parser([
                    PossibleValue::new("trace"),
                    PossibleValue::new("debug"),
                    PossibleValue::new("info"),
                    PossibleValue::new("warn"),
                    PossibleValue::new("error"),
                ])
                .help("Log level")
                .global(true)
        )
        .arg_required_else_help(true)
        .subcommand(Command::new("server"))
        .get_matches();

    let log_level: &String = matches.get_one("log").unwrap();
    env_logger::init_from_env(Env::new()
        .filter("MINECRUST_LOG")
        .default_filter_or(log_level)
    );
    log::info!("Logger initialized with level {}", log_level);

    match matches.subcommand() {
        Some(("server", args)) => {
            let host: &SocketAddr = args.get_one("addr").unwrap();

            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()?
                .block_on(async {
                    server::start(host).await?;

                    Ok::<_, ServerError>(())
                })?;
        }
        _ => {}
    }
    Ok(())
}
