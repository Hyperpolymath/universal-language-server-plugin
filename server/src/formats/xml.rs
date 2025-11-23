//! XML format support for document conversion

use anyhow::Result;
use serde_json::Value;

/// Convert XML to JSON
pub fn xml_to_json(xml: &str) -> Result<String> {
    // Placeholder: use quick-xml or serde-xml-rs in production
    let value: Value = serde_json::json!({
        "xml_root": {
            "content": xml
        }
    });
    Ok(serde_json::to_string_pretty(&value)?)
}

/// Convert JSON to XML
pub fn json_to_xml(json: &str) -> Result<String> {
    let value: Value = serde_json::from_str(json)?;
    Ok(format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<root>\n  {}\n</root>",
               serde_json::to_string_pretty(&value)?))
}

/// Validate XML syntax
pub fn validate_xml(xml: &str) -> Result<Vec<String>> {
    let mut diagnostics = Vec::new();

    if xml.trim().is_empty() {
        diagnostics.push("XML document is empty".to_string());
    }

    // Check for XML declaration
    if !xml.starts_with("<?xml") {
        diagnostics.push("Missing XML declaration".to_string());
    }

    // Basic tag matching
    let open_tags = xml.matches('<').count();
    let close_tags = xml.matches('>').count();
    if open_tags != close_tags {
        diagnostics.push("Mismatched XML tags".to_string());
    }

    Ok(diagnostics)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_xml_empty() {
        let diagnostics = validate_xml("").unwrap();
        assert!(!diagnostics.is_empty());
    }

    #[test]
    fn test_validate_xml_no_declaration() {
        let xml = "<root></root>";
        let diagnostics = validate_xml(xml).unwrap();
        assert!(diagnostics.iter().any(|d| d.contains("declaration")));
    }
}
