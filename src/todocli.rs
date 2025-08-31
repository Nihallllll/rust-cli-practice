use clap::{Parser, Subcommand};
use shlex;
use dotenvy::dotenv;
use postgres::Client;
use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::MakeTlsConnector;
use std::env;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(no_binary_name = true)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Debug, Subcommand)]

enum Commands {
    Add {
        name: String,
        time: String,
        #[arg(long)]
        is_done: bool,
    },
    Del {
        name: String,
    },
}

pub fn todo() {
    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .expect("Couldn't parse stdin");

    let line = buf.trim();
    let args = shlex::split(line).ok_or("error: Invalid quoting").unwrap();

    println!("{:?}", args);
    // let tasks =
    match Args::try_parse_from(args) {
        Ok(cli) => match cli.cmd {
            Commands::Add {
                name,
                time,
                is_done 
            } => println!("the task {:?} is saved", name),
            Commands::Del { name } => {
                println!("task {:?} is been deleted", name)
            }
        },
        Err(_) => println!("That's not a valid command - use the help command if you are stuck."),
    }
}
