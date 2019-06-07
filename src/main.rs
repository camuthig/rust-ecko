use quicli::prelude::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "ecko",
    raw(setting = "structopt::clap::AppSettings::SubcommandRequiredElseHelp"))
]
struct Cli {
    #[structopt(short = "p")]
    /// The punctuation to add to the printed statement
    punctuation: Option<String>,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "hello")]
    Hello(Hello),
    #[structopt(name = "goodbye")]
    Goodbye(Goodbye),
}

#[derive(Debug, StructOpt)]
struct Hello {
    /// The name to say hello to
    #[structopt(name = "who")]
    who: String,
}

#[derive(Debug, StructOpt)]
struct Goodbye {
    /// The name to say goodbye to
    #[structopt(name = "who")]
    who: String,
}

fn main() -> CliResult {
    let cli = Cli::from_args();

    match &cli.cmd {
        Command::Hello(c) => hello(&cli, &c)?,
        Command::Goodbye(c) => goodbye(&cli, &c)?,
    }

    Ok(())
}

fn hello(cli: &Cli, cmd: &Hello) -> Result<(), Error> {
    println!("Hello, {}{}", cmd.who, cli.punctuation.as_ref().unwrap_or(&String::new()));
    Ok(())
}

fn goodbye(cli: &Cli, cmd: &Goodbye) -> Result<(), Error> {
    println!("Goodbye, {}{}",cmd.who, cli.punctuation.as_ref().unwrap_or(&String::new()));
    Ok(())
}
