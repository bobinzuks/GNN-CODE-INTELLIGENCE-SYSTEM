//! Language-specific parser implementations
//!
//! This module provides a trait-based system for implementing parsers
//! for different programming languages.

use crate::graph::CodeGraphBuilder;
use anyhow::Result;
use std::path::Path;

pub mod rust;
pub mod python;
pub mod javascript;
pub mod typescript;
pub mod go;
pub mod java;
pub mod cpp;
pub mod c;
pub mod swift;

/// Trait for language-specific parsers
pub trait LanguageParser: Send + Sync {
    /// Get the language name
    fn language_name(&self) -> &str;

    /// Get file extensions this parser handles
    fn file_extensions(&self) -> &[&str];

    /// Parse a single file and add nodes/edges to the graph builder
    fn parse_file(&self, file_path: &Path, builder: &mut CodeGraphBuilder) -> Result<()>;

    /// Check if this parser can handle the given file
    fn can_parse(&self, file_path: &Path) -> bool {
        if let Some(ext) = file_path.extension() {
            let ext_str = ext.to_string_lossy();
            self.file_extensions().iter().any(|&e| e == ext_str)
        } else {
            false
        }
    }
}

/// Registry of language parsers
pub struct ParserRegistry {
    parsers: Vec<Box<dyn LanguageParser>>,
}

impl ParserRegistry {
    /// Create a new parser registry with all supported parsers
    pub fn new() -> Self {
        let mut registry = Self {
            parsers: Vec::new(),
        };

        // Register all available parsers
        registry.register(Box::new(rust::RustParser::new()));
        registry.register(Box::new(python::PythonParser::new()));
        registry.register(Box::new(javascript::JavaScriptParser::new()));
        registry.register(Box::new(typescript::TypeScriptParser::new()));
        registry.register(Box::new(go::GoParser::new()));
        registry.register(Box::new(java::JavaParser::new()));
        registry.register(Box::new(cpp::CppParser::new()));
        registry.register(Box::new(c::CParser::new()));
        registry.register(Box::new(swift::SwiftParser::new()));

        registry
    }

    /// Register a new parser
    pub fn register(&mut self, parser: Box<dyn LanguageParser>) {
        self.parsers.push(parser);
    }

    /// Find a parser for the given file
    pub fn find_parser(&self, file_path: &Path) -> Option<&dyn LanguageParser> {
        self.parsers
            .iter()
            .find(|p| p.can_parse(file_path))
            .map(|p| p.as_ref())
    }

    /// Get all registered parsers
    pub fn parsers(&self) -> &[Box<dyn LanguageParser>] {
        &self.parsers
    }

    /// Get supported file extensions
    pub fn supported_extensions(&self) -> Vec<&str> {
        self.parsers
            .iter()
            .flat_map(|p| p.file_extensions())
            .copied()
            .collect()
    }
}

impl Default for ParserRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_parser_registry() {
        let registry = ParserRegistry::new();
        assert!(!registry.parsers.is_empty());

        let rust_file = PathBuf::from("test.rs");
        assert!(registry.find_parser(&rust_file).is_some());

        let unknown_file = PathBuf::from("test.unknown");
        assert!(registry.find_parser(&unknown_file).is_none());
    }

    #[test]
    fn test_supported_extensions() {
        let registry = ParserRegistry::new();
        let extensions = registry.supported_extensions();
        assert!(extensions.contains(&"rs"));
    }
}
