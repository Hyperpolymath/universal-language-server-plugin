//! TOML format support for document conversion

use anyhow::Result;
use serde_json::Value;

/// Convert TOML to JSON
pub fn toml_to_json(toml: &str) -> Result<String> {
    // Placeholder: use toml crate in production
    let value: Value = serde_json::json!({
        "toml_content": toml
    });
    Ok(serde_json::to_string_pretty(&value)?)
}

/// Convert JSON to TOML
pub fn json_to_toml(json: &str) -> Result<String> {
    let value: Value = serde_json::from_str(json)?;
    // Placeholder: use toml crate for proper serialization
    Ok(format!("# TOML\n[data]\ncontent = '''{}'''", value))
}

/// Validate TOML syntax
pub fn validate_toml(toml: &str) -> Result<Vec<String>> {
    let mut diagnostics = Vec::new();

    if toml.trim().is_empty() {
        diagnostics.push("TOML document is empty".to_string());
    }

    // Check for common TOML issues
    if toml.contains('\t') && !toml.contains("'''") {
        diagnostics.push("TOML should use spaces for indentation outside of strings".to_string());
    }

    Ok(diagnostics)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toml_to_json() {
        let toml = "key = \"value\"";
        let result = toml_to_json(toml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_toml_empty() {
        let diagnostics = validate_toml("").unwrap();
        assert!(!diagnostics.is_empty());
    }
}
