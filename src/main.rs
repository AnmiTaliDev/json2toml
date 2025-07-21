use clap::Parser;
use colored::*;
use std::path::PathBuf;
use anyhow::Result;

mod to_json;
mod to_toml;

#[derive(Parser)]
#[command(name = "json2toml")]
#[command(about = "A CLI utility to convert between JSON and TOML formats")]
#[command(version = "0.1.0")]
#[command(author = "AnmiTalIDev <anmitali198@gmail.com>")]
struct Cli {
    /// Input file path
    input: PathBuf,
    
    /// Output file path (optional, prints to stdout if not provided)
    #[arg(short, long)]
    output: Option<PathBuf>,
    
    /// Pretty print the output
    #[arg(short, long, default_value_t = false)]
    pretty: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Determine conversion direction based on file extension
    let input_ext = cli.input.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");
    
    match input_ext.to_lowercase().as_str() {
        "json" => {
            println!("{}", "Converting JSON to TOML...".green());
            to_toml::convert_json_to_toml(&cli.input, cli.output.as_deref(), cli.pretty)?;
            println!("{}", "Conversion completed successfully!".bright_green());
        }
        "toml" => {
            println!("{}", "Converting TOML to JSON...".cyan());
            to_json::convert_toml_to_json(&cli.input, cli.output.as_deref(), cli.pretty)?;
            println!("{}", "Conversion completed successfully!".bright_cyan());
        }
        _ => {
            eprintln!("{}", "Error: Input file must have .json or .toml extension".red());
            std::process::exit(1);
        }
    }
    
    Ok(())
}