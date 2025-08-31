use clap::{Parser, Subcommand};
use shlex;
use dotenvy::dotenv;
use postgres::{Client, NoTls};
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
        task: String,
        #[arg(long, default_value = "mid")]
        priority: String,
        #[arg(long, default_value_t = false)]
        is_done: bool,
    },
    Del {
        id: i32,
    },
    List,
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let conn_string = env::var("DATABASE_URL")?;
    println!("DATABASE_URL = {:?}", std::env::var("DATABASE_URL"));

    let builder = SslConnector::builder(SslMethod::tls())?;
    let connector = MakeTlsConnector::new(builder.build());
    let mut client = Client::connect(&conn_string, connector)?;

    println!("Connection established");
client.batch_execute("CREATE TYPE IF NOT EXISTS priority_level AS ENUM ('low', 'mid', 'high');")?;
client.batch_execute("CREATE TABLE IF NOT EXISTS users (id SERIAL PRIMARY KEY);")?;
client.batch_execute("CREATE TABLE IF NOT EXISTS todos (id SERIAL PRIMARY KEY, user_id INT REFERENCES users(id) ON DELETE CASCADE, task TEXT NOT NULL, priority priority_level NOT NULL, is_done BOOLEAN DEFAULT FALSE);")?;

    println!("Finished creating tables.");

    // Make sure at least one user exists
    let row = client.query_one("INSERT INTO users DEFAULT VALUES RETURNING id;", &[])?;
    let user_id: i32 = row.get(0);

    // Read CLI input
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Couldn't parse stdin");
    let line = buf.trim();
    let args = shlex::split(line).ok_or("error: Invalid quoting")?;

    match Args::try_parse_from(args) {
        Ok(cli) => match cli.cmd {
            Commands::Add { task, priority, is_done } => {
                client.execute(
                    "INSERT INTO todos (user_id, task, priority, is_done) VALUES ($1, $2, $3::priority_level, $4)",
                    &[&user_id, &task, &priority, &is_done],
                )?;
                println!("✅ Task '{}' added with priority {}", task, priority);
            }
            Commands::Del { id } => {
                let rows = client.execute("DELETE FROM todos WHERE id = $1", &[&id])?;
                if rows > 0 {
                    println!("🗑️ Task {} deleted", id);
                } else {
                    println!("⚠️ No task found with id {}", id);
                }
            }
            Commands::List => {
                for row in client.query("SELECT id, task, priority, is_done FROM todos", &[])? {
                    let id: i32 = row.get(0);
                    let task: String = row.get(1);
                    let priority: String = row.get(2);
                    let is_done: bool = row.get(3);
                    println!(
                        "[{}] {} (priority: {}, done: {})",
                        id, task, priority, is_done
                    );
                }
            }
        },
        Err(_) => println!("❌ Invalid command - try 'add', 'del', or 'list'."),
    }

    Ok(())
}
