use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Cli ")]
#[command(about="Cli toh cli h na ")]

struct Cli{
    #[command(subcommand)]
    command : Commands
}

#[derive(Subcommand)]
enum Commands{
    Hello {
        name : String
    },
    mul {
        a:u32,
        b:u32
    }
}

pub fn cliofcpi(){

    let cli = Cli::parse();

    match cli.command{
        Commands::Hello{name} => {println!("Hello {}",name)}
        Commands::mul { a, b } => {println!("multiplying  {} and {} is {}",a,b,a*b)}
    }
}