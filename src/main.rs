use clap::{Parser, Subcommand};

/// Print your day: calendar events, assigned GitHub items and local tasks.
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Show today's brief.
    Today,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Today => println!("daybrief: nothing to show yet"),
    }
}
