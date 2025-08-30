use clap::{Parser, Subcommand};
use shlex;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Get {
        key: String,
    },
    Set {
        key: String,
        value: String,
        #[arg(long)]
        is_true: bool,
    },
    // Remove Help to avoid duplicate help command panic
}

pub fn clapcli2() {
    loop {
        let mut buf = String::new();
        std::io::stdin()
            .read_line(&mut buf)
            .expect("Couldn't parse stdin");

        let line = buf.trim();
        let mut args = vec!["program".to_string()]; // dummy binary name required by clap
        match shlex::split(line) {
            Some(mut split_args) => args.append(&mut split_args),
            None => {
                println!("error: Invalid quoting");
                continue;
            }
        }

        println!("{:?}", args);

        match Args::try_parse_from(args) {
            Ok(cli) => match cli.cmd {
                Commands::Get { key } => println!("get {:?}", key),
                Commands::Set { key, value, is_true } => {
                    println!("set {} = {}, is_true = {}", key, value, is_true)
                }
            },
            Err(_) => println!("That's not a valid command - use the help command if you are stuck."),
        };
    }
}

fn main() {
    clapcli2();
}
