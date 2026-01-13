use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use std::io::{self, BufRead};

#[derive(Parser)]
#[command(name = "b58uuid")]
#[command(author, version)]
#[command(about = "Compact Base58 UUID Encoder - Convert UUIDs to 22-character format")]
#[command(long_about = "B58UUID CLI converts standard UUIDs (36 characters) to compact Base58 format (22 characters).\n\
This reduces storage size by 39% while maintaining URL-safety and readability.\n\n\
The tool supports encoding, decoding, generation, and validation of both UUID and B58UUID formats.")]
#[command(after_help = "EXAMPLES:\n  \
    # Encode a UUID to B58UUID\n  \
    b58uuid encode 550e8400-e29b-41d4-a716-446655440000\n  \
    b58uuid enc 550e8400-e29b-41d4-a716-446655440000  # Using alias\n\n  \
    # Decode a B58UUID to UUID\n  \
    b58uuid decode BWBeN28Vb7cMEx7Ym8AUzs\n  \
    b58uuid dec BWBeN28Vb7cMEx7Ym8AUzs  # Using alias\n\n  \
    # Generate random B58UUIDs\n  \
    b58uuid generate\n  \
    b58uuid gen --count 5  # Generate 5 B58UUIDs\n  \
    b58uuid gen --uuid     # Generate as standard UUID\n\n  \
    # Validate format\n  \
    b58uuid validate 550e8400-e29b-41d4-a716-446655440000\n  \
    b58uuid val BWBeN28Vb7cMEx7Ym8AUzs  # Using alias\n\n  \
    # Batch processing from stdin\n  \
    echo '550e8400-e29b-41d4-a716-446655440000' | b58uuid encode\n  \
    cat uuids.txt | b58uuid encode\n\n  \
    # Batch processing from file\n  \
    b58uuid encode --file uuids.txt\n  \
    b58uuid decode --file b58uuids.txt\n\n  \
    # Disable colors\n  \
    b58uuid --no-color encode 550e8400-e29b-41d4-a716-446655440000\n\n\
ALIASES:\n  \
    encode    -> enc\n  \
    decode    -> dec\n  \
    generate  -> gen\n  \
    validate  -> val\n\n\
For more information, visit: https://b58uuid.io")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Disable colored output
    #[arg(long, global = true, help = "Disable colored output for piping or logging")]
    no_color: bool,

    /// Output format (currently only 'text' is supported)
    #[arg(short, long, global = true, default_value = "text", hide = true)]
    format: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Encode UUID to B58UUID format (alias: enc)
    /// 
    /// Converts a standard UUID (36 characters) to compact B58UUID format (22 characters).
    /// Supports single UUID, batch processing from stdin, or file input.
    #[command(alias = "enc")]
    #[command(after_help = "EXAMPLES:\n  \
        b58uuid encode 550e8400-e29b-41d4-a716-446655440000\n  \
        echo '550e8400-e29b-41d4-a716-446655440000' | b58uuid enc\n  \
        b58uuid encode --file uuids.txt")]
    Encode {
        /// UUID to encode (reads from stdin if not provided)
        /// 
        /// Format: xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx (36 characters)
        /// Example: 550e8400-e29b-41d4-a716-446655440000
        uuid: Option<String>,

        /// Read UUIDs from file (one per line)
        #[arg(short, long, value_name = "FILE")]
        file: Option<String>,
    },

    /// Decode B58UUID to standard UUID format (alias: dec)
    /// 
    /// Converts a B58UUID (22 characters) back to standard UUID format (36 characters).
    /// Supports single B58UUID, batch processing from stdin, or file input.
    #[command(alias = "dec")]
    #[command(after_help = "EXAMPLES:\n  \
        b58uuid decode BWBeN28Vb7cMEx7Ym8AUzs\n  \
        echo 'BWBeN28Vb7cMEx7Ym8AUzs' | b58uuid dec\n  \
        b58uuid decode --file b58uuids.txt")]
    Decode {
        /// B58UUID to decode (reads from stdin if not provided)
        /// 
        /// Format: 22 Base58 characters (no 0, O, I, l)
        /// Example: BWBeN28Vb7cMEx7Ym8AUzs
        b58uuid: Option<String>,

        /// Read B58UUIDs from file (one per line)
        #[arg(short, long, value_name = "FILE")]
        file: Option<String>,
    },

    /// Generate random B58UUID or UUID (alias: gen)
    /// 
    /// Generates one or more random UUIDs in B58UUID or standard UUID format.
    /// Uses UUID v4 (random) generation.
    #[command(alias = "gen")]
    #[command(after_help = "EXAMPLES:\n  \
        b58uuid generate              # Generate one B58UUID\n  \
        b58uuid gen -n 5              # Generate 5 B58UUIDs\n  \
        b58uuid gen --count 10        # Generate 10 B58UUIDs\n  \
        b58uuid gen --uuid            # Generate as standard UUID\n  \
        b58uuid gen -n 5 --uuid       # Generate 5 standard UUIDs")]
    Generate {
        /// Number of UUIDs to generate
        #[arg(short = 'n', long, default_value = "1", value_name = "COUNT")]
        count: usize,

        /// Output as standard UUID instead of B58UUID
        /// 
        /// By default, generates B58UUID format (22 chars).
        /// Use this flag to generate standard UUID format (36 chars).
        #[arg(short, long)]
        uuid: bool,
    },

    /// Validate UUID or B58UUID format (alias: val)
    /// 
    /// Checks if the input is a valid UUID or B58UUID and displays both formats.
    /// Returns exit code 0 for valid input, 1 for invalid input.
    #[command(alias = "val")]
    #[command(after_help = "EXAMPLES:\n  \
        b58uuid validate 550e8400-e29b-41d4-a716-446655440000\n  \
        b58uuid val BWBeN28Vb7cMEx7Ym8AUzs\n  \
        b58uuid validate invalid-format  # Returns exit code 1")]
    Validate {
        /// Value to validate (UUID or B58UUID)
        /// 
        /// Accepts either:
        ///   - Standard UUID: 36 characters with hyphens
        ///   - B58UUID: 22 Base58 characters
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
