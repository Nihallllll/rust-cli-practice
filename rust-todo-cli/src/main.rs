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

pub fn main() {
   dotenv()?;
    let conn_string = env::var("DATABASE_URL")?;
    let builder = SslConnector::builder(SslMethod::tls())?;
    let connector = MakeTlsConnector::new(builder.build());
    let mut client = Client::connect(&conn_string, connector)?;
    println!("Connection established");
    client.batch_execute("DROP TABLE IF EXISTS todos;")?;
    println!("Finished dropping table (if it existed).");
    client.batch_execute(
    "
    CREATE TABLE IF NOT EXISTS users (
        id SERIAL PRIMARY KEY
    );

    CREATE TYPE IF NOT EXISTS priority_level AS ENUM ('low', 'mid', 'high');

    CREATE TABLE IF NOT EXISTS todos (
        id SERIAL PRIMARY KEY,
        user_id INT REFERENCES users(id) ON DELETE CASCADE,
        task TEXT NOT NULL,
        priority priority_level NOT NULL,
        is_done BOOLEAN DEFAULT FALSE
    );
    "
)?;
    println!("Finished creating table.");
    // Insert a single book record
    client.execute(
        "INSERT INTO books (title, author, publication_year, in_stock) VALUES ($1, $2, $3, $4)",
        &[&"The Catcher in the Rye", &"J.D. Salinger", &1951, &true],
    )?;
    println!("Inserted a single book.");
    // Start a transaction
    let mut transaction = client.transaction()?;
    println!("Starting transaction to insert multiple books...");


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
