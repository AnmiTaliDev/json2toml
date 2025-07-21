use anyhow::{Context, Result};
use serde_json::Value;
use std::fs;
use std::path::Path;
use colored::*;

pub fn convert_toml_to_json(
    input_path: &Path,
    output_path: Option<&Path>,
    pretty: bool,
) -> Result<()> {
    // Read the TOML file
    let toml_content = fs::read_to_string(input_path)
        .with_context(|| format!("Failed to read TOML file: {}", input_path.display()))?;
    
    // Parse TOML
    let toml_value: Value = toml::from_str(&toml_content)
        .with_context(|| "Failed to parse TOML content")?;
    
    // Convert to JSON string
    let json_string = if pretty {
        serde_json::to_string_pretty(&toml_value)
            .with_context(|| "Failed to convert TOML to pretty JSON")?
    } else {
        serde_json::to_string(&toml_value)
            .with_context(|| "Failed to convert TOML to JSON")?
    };
    
    // Output the result
    match output_path {
        Some(path) => {
            fs::write(path, &json_string)
                .with_context(|| format!("Failed to write JSON file: {}", path.display()))?;
            println!("📝 Output written to: {}", path.display().to_string().yellow());
        }
        None => {
            println!("{}", "📋 JSON Output:".blue());
            println!("{}", json_string);
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_simple_toml_to_json_conversion() {
        let toml_content = r#"name = "test"
version = "1.0.0"
debug = true"#;
        
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "{}", toml_content).unwrap();
        
        let result = convert_toml_to_json(temp_file.path(), None, false);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_nested_toml_to_json_conversion() {
        let toml_content = r#"[package]
name = "test-package"
version = "1.0.0"
authors = ["author1", "author2"]

[dependencies]
serde = "1.0"
tokio = "1.0""#;
        
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "{}", toml_content).unwrap();
        
        let result = convert_toml_to_json(temp_file.path(), None, false);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_pretty_json_output() {
        let toml_content = r#"name = "test"
version = "1.0.0""#;
        
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "{}", toml_content).unwrap();
        
        let result = convert_toml_to_json(temp_file.path(), None, true);
        assert!(result.is_ok());
    }
}