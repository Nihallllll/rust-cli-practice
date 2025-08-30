use clap::{Parser, Subcommand};
use shlex;
#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
#[command(no_binary_name = true)]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Debug, Subcommand)]

enum Commands {
    Add {
        name:String,
        time: String,
        is_done:bool,
    }

}
