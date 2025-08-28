use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Get(String),
    Set {
        key: String,
        value: String,
        is_true: bool
    },
    Help
}

pub fn clapcli2() {
    loop {
        let mut buf = String::from(crate_name!());
        
        std::io::stdin().read_line(&mut buf).expect("Couldn't parse stdin");
        let line = buf.trim();
        let args = shlex::split(line).ok_or("error: Invalid quoting").unwrap();

        println!("{:?}", args);

        match Args::try_parse_from(args.iter()).map_err(|e| e.to_string()) {
            Ok(cli) => {
                match cli.cmd {
                    Commands::Get(value) => {println!("get")},
                    Commands::Set{key, value, is_true} => {println!("get")},
                    Commands::Help => {println!("get")},
                }
            }
            Err(_) => println!("That's not a valid command - use the help command if you are stuck.");
         };
    }
}
