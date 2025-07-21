use anyhow::{Context, Result};
use serde_json::Value;
use std::fs;
use std::path::Path;
use colored::*;

pub fn convert_json_to_toml(
    input_path: &Path,
    output_path: Option<&Path>,
    _pretty: bool,
) -> Result<()> {
    // Read the JSON file
    let json_content = fs::read_to_string(input_path)
        .with_context(|| format!("Failed to read JSON file: {}", input_path.display()))?;
    
    // Parse JSON
    let json_value: Value = serde_json::from_str(&json_content)
        .with_context(|| "Failed to parse JSON content")?;
    
    // Convert to TOML
    let toml_string = toml::to_string(&json_value)
        .with_context(|| "Failed to convert JSON to TOML")?;
    
    // Output the result
    match output_path {
        Some(path) => {
            fs::write(path, &toml_string)
                .with_context(|| format!("Failed to write TOML file: {}", path.display()))?;
            println!("📝 Output written to: {}", path.display().to_string().yellow());
        }
        None => {
            println!("{}", "📋 TOML Output:".blue());
            println!("{}", toml_string);
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
    fn test_simple_json_to_toml_conversion() {
        let json_content = r#"{"name": "test", "version": "1.0.0", "debug": true}"#;
        
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "{}", json_content).unwrap();
        
        let result = convert_json_to_toml(temp_file.path(), None, false);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_nested_json_to_toml_conversion() {
        let json_content = r#"{
            "package": {
                "name": "test-package",
                "version": "1.0.0",
                "authors": ["author1", "author2"]
            },
            "dependencies": {
                "serde": "1.0",
                "tokio": "1.0"
            }
        }"#;
        
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "{}", json_content).unwrap();
        
        let result = convert_json_to_toml(temp_file.path(), None, false);
        assert!(result.is_ok());
    }
}