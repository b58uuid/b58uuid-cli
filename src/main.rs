use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use std::io::{self, BufRead};

#[derive(Parser)]
#[command(name = "b58uuid")]
#[command(author, version, about = "Compact Base58 UUID Encoder", long_about = None)]
#[command(after_help = "Examples:\n  \
    b58uuid encode 550e8400-e29b-41d4-a716-446655440000\n  \
    b58uuid decode BWBeN28Vb7cMEx7Ym8AUzs\n  \
    b58uuid generate\n  \
    b58uuid generate --count 5\n  \
    echo '550e8400-e29b-41d4-a716-446655440000' | b58uuid encode\n\n\
For more information, visit: https://b58uuid.io")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Disable colored output
    #[arg(long, global = true)]
    no_color: bool,

    /// Output format (text, json)
    #[arg(short, long, global = true, default_value = "text")]
    format: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Encode UUID to B58UUID format
    #[command(alias = "enc")]
    Encode {
        /// UUID to encode (reads from stdin if not provided)
        uuid: Option<String>,

        /// Read from file
        #[arg(short, long)]
        file: Option<String>,
    },

    /// Decode B58UUID to standard UUID format
    #[command(alias = "dec")]
    Decode {
        /// B58UUID to decode (reads from stdin if not provided)
        b58uuid: Option<String>,

        /// Read from file
        #[arg(short, long)]
        file: Option<String>,
    },

    /// Generate random B58UUID
    #[command(alias = "gen")]
    Generate {
        /// Number of UUIDs to generate
        #[arg(short = 'n', long, default_value = "1")]
        count: usize,

        /// Output as standard UUID instead of B58UUID
        #[arg(short, long)]
        uuid: bool,
    },

    /// Validate UUID or B58UUID format
    #[command(alias = "val")]
    Validate {
        /// Value to validate
        value: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Disable colors if requested
    if cli.no_color {
        colored::control::set_override(false);
    }

    match cli.command {
        Commands::Encode { uuid, file } => {
            if let Some(file_path) = file {
                encode_from_file(&file_path)?;
            } else if let Some(uuid_str) = uuid {
                encode_single(&uuid_str)?;
            } else {
                encode_from_stdin()?;
            }
        }
        Commands::Decode { b58uuid, file } => {
            if let Some(file_path) = file {
                decode_from_file(&file_path)?;
            } else if let Some(b58_str) = b58uuid {
                decode_single(&b58_str)?;
            } else {
                decode_from_stdin()?;
            }
        }
        Commands::Generate { count, uuid } => {
            generate_uuids(count, uuid)?;
        }
        Commands::Validate { value } => {
            validate_value(&value)?;
        }
    }

    Ok(())
}

fn encode_single(uuid_str: &str) -> Result<()> {
    let uuid_str = uuid_str.trim();
    match b58uuid::encode_uuid(uuid_str) {
        Ok(b58) => {
            println!("{}", b58.green());
        }
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    }
    Ok(())
}

fn encode_from_stdin() -> Result<()> {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.context("Failed to read from stdin")?;
        let uuid_str = line.trim();
        if !uuid_str.is_empty() {
            match b58uuid::encode_uuid(uuid_str) {
                Ok(b58) => println!("{}", b58.green()),
                Err(e) => eprintln!("{} {} - {}", "Error:".red().bold(), uuid_str, e),
            }
        }
    }
    Ok(())
}

fn encode_from_file(file_path: &str) -> Result<()> {
    let content = std::fs::read_to_string(file_path)
        .context(format!("Failed to read file: {}", file_path))?;

    for line in content.lines() {
        let uuid_str = line.trim();
        if !uuid_str.is_empty() {
            match b58uuid::encode_uuid(uuid_str) {
                Ok(b58) => println!("{}", b58.green()),
                Err(e) => eprintln!("{} {} - {}", "Error:".red().bold(), uuid_str, e),
            }
        }
    }
    Ok(())
}

fn decode_single(b58_str: &str) -> Result<()> {
    let b58_str = b58_str.trim();
    match b58uuid::decode_to_uuid(b58_str) {
        Ok(uuid) => {
            println!("{}", uuid.green());
        }
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    }
    Ok(())
}

fn decode_from_stdin() -> Result<()> {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.context("Failed to read from stdin")?;
        let b58_str = line.trim();
        if !b58_str.is_empty() {
            match b58uuid::decode_to_uuid(b58_str) {
                Ok(uuid) => println!("{}", uuid.green()),
                Err(e) => eprintln!("{} {} - {}", "Error:".red().bold(), b58_str, e),
            }
        }
    }
    Ok(())
}

fn decode_from_file(file_path: &str) -> Result<()> {
    let content = std::fs::read_to_string(file_path)
        .context(format!("Failed to read file: {}", file_path))?;

    for line in content.lines() {
        let b58_str = line.trim();
        if !b58_str.is_empty() {
            match b58uuid::decode_to_uuid(b58_str) {
                Ok(uuid) => println!("{}", uuid.green()),
                Err(e) => eprintln!("{} {} - {}", "Error:".red().bold(), b58_str, e),
            }
        }
    }
    Ok(())
}

fn generate_uuids(count: usize, as_uuid: bool) -> Result<()> {
    for _ in 0..count {
        if as_uuid {
            let uuid = uuid::Uuid::new_v4();
            println!("{}", uuid.to_string().green());
        } else {
            let b58 = b58uuid::generate();
            println!("{}", b58.green());
        }
    }
    Ok(())
}

fn validate_value(value: &str) -> Result<()> {
    let value = value.trim();

    // Try to decode as B58UUID
    if let Ok(uuid) = b58uuid::decode_to_uuid(value) {
        println!("{} Valid B58UUID", "✓".green().bold());
        println!("  B58UUID: {}", value.cyan());
        println!("  UUID:    {}", uuid.cyan());
        return Ok(());
    }

    // Try to encode as UUID
    if let Ok(b58) = b58uuid::encode_uuid(value) {
        println!("{} Valid UUID", "✓".green().bold());
        println!("  UUID:    {}", value.cyan());
        println!("  B58UUID: {}", b58.cyan());
        return Ok(());
    }

    eprintln!("{} Invalid format", "✗".red().bold());
    eprintln!("  Value: {}", value);
    eprintln!("  Expected: UUID (36 chars) or B58UUID (22 chars)");
    std::process::exit(1);
}
