pub mod subcommands;

use std::io::{self, Write};

use crate::subcommands::Commands;
use clap::Parser;

const BANNER: &str = r#"

        ██████╗██╗███╗   ██╗███████╗██╗     ██╗███╗   ██╗███████╗
        ██╔════╝██║████╗  ██║██╔════╝██║     ██║████╗  ██║██╔════╝
        ██║     ██║██╔██╗ ██║█████╗  ██║     ██║██╔██╗ ██║█████╗  
        ██║     ██║██║╚██╗██║██╔══╝  ██║     ██║██║╚██╗██║██╔══╝  
        ╚██████╗██║██║ ╚████║███████╗███████╗██║██║ ╚████║███████╗
        ╚═════╝╚═╝╚═╝  ╚═══╝╚══════╝╚══════╝╚═╝╚═╝  ╚═══╝╚══════╝

"#;

#[derive(Parser)]
#[command(name = "cineline", disable_help_subcommand = true)]
#[command(about = "A Rust CLI tool to browse, list, and stream available movies in your default browser", long_about = None)]
#[command(after_help = "Type 'quit' or 'exit' to leave the CLI.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() {
    print_banner();

    loop {
        let prompt = "cineline> ";
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" || input == "quit" {
            break;
        }

        let mut args = vec!["cineline"];
        args.extend(input.split_whitespace());

        match Cli::try_parse_from(args) {
            Ok(cli) => match cli.command {
               Commands::Test {  } => {
                    println!("Here!");
               }
            },
            Err(e) => {
                println!("\tAin't a command, partner! \n \t -> {}", e);
            }
        }
    }

    let farewell_text = "'Till next time.";
    println!("\n {} \n", farewell_text);
}

fn print_banner() {
    println!("{}", BANNER);
}
