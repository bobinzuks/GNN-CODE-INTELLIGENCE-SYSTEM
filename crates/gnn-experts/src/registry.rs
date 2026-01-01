//! Expert registry for dynamic loading and management of language experts

use crate::trait_::LanguageExpert;
use crate::experts::rust::RustExpert;
use std::collections::HashMap;
use std::sync::Arc;

/// Registry for managing language experts
pub struct ExpertRegistry {
    experts: HashMap<String, Arc<dyn LanguageExpert>>,
}

impl ExpertRegistry {
    /// Create a new registry with default experts
    pub fn new() -> Self {
        let mut registry = Self {
            experts: HashMap::new(),
        };

        // Register built-in experts
        registry.register_default_experts();

        registry
    }

    /// Create an empty registry
    pub fn empty() -> Self {
        Self {
            experts: HashMap::new(),
        }
    }

    /// Register default language experts
    fn register_default_experts(&mut self) {
        // Register Rust expert
        self.register_expert(Arc::new(RustExpert::new()));

        // Note: Other experts (C++, Go, TypeScript, etc.) would be registered here
        // when they are implemented
    }

    /// Register a language expert
    pub fn register_expert(&mut self, expert: Arc<dyn LanguageExpert>) {
        let language = expert.language().to_lowercase();
        self.experts.insert(language, expert);
    }

    /// Get an expert for a specific language
    pub fn get(&self, language: &str) -> Option<Arc<dyn LanguageExpert>> {
        let language_key = language.to_lowercase();
        self.experts.get(&language_key).cloned()
    }

    /// Check if an expert is registered for a language
    pub fn has_expert(&self, language: &str) -> bool {
        let language_key = language.to_lowercase();
        self.experts.contains_key(&language_key)
    }

    /// Get all registered language names
    pub fn languages(&self) -> Vec<String> {
        self.experts.keys().cloned().collect()
    }

    /// Get all registered experts
    pub fn all_experts(&self) -> Vec<Arc<dyn LanguageExpert>> {
        self.experts.values().cloned().collect()
    }

    /// Get expert count
    pub fn count(&self) -> usize {
        self.experts.len()
    }

    /// Remove an expert
    pub fn remove(&mut self, language: &str) -> Option<Arc<dyn LanguageExpert>> {
        let language_key = language.to_lowercase();
        self.experts.remove(&language_key)
    }

    /// Clear all experts
    pub fn clear(&mut self) {
        self.experts.clear();
    }

    /// Find experts that can handle a specific language
    pub fn find_compatible(&self, language: &str) -> Vec<Arc<dyn LanguageExpert>> {
        self.experts
            .values()
            .filter(|expert| expert.can_handle(language))
            .cloned()
            .collect()
    }
}

impl Default for ExpertRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for ExpertRegistry
pub struct ExpertRegistryBuilder {
    registry: ExpertRegistry,
}

impl ExpertRegistryBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            registry: ExpertRegistry::empty(),
        }
    }

    /// Add an expert
    pub fn with_expert(mut self, expert: Arc<dyn LanguageExpert>) -> Self {
        self.registry.register_expert(expert);
        self
    }

    /// Add default experts
    pub fn with_defaults(mut self) -> Self {
        self.registry.register_default_experts();
        self
    }

    /// Build the registry
    pub fn build(self) -> ExpertRegistry {
        self.registry
    }
}

impl Default for ExpertRegistryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let registry = ExpertRegistry::new();
        assert!(registry.count() > 0, "Registry should have default experts");
    }

    #[test]
    fn test_empty_registry() {
        let registry = ExpertRegistry::empty();
        assert_eq!(registry.count(), 0);
    }

    #[test]
    fn test_register_and_get_expert() {
        let mut registry = ExpertRegistry::empty();
        let expert = Arc::new(RustExpert::new());
        registry.register_expert(expert);

        assert!(registry.has_expert("rust"));
        assert!(registry.get("rust").is_some());
        assert!(registry.get("Rust").is_some()); // Case insensitive
        assert!(registry.get("cpp").is_none());
    }

    #[test]
    fn test_get_all_languages() {
        let registry = ExpertRegistry::new();
        let languages = registry.languages();
        assert!(languages.contains(&"rust".to_string()));
    }

    #[test]
    fn test_remove_expert() {
        let mut registry = ExpertRegistry::new();
        assert!(registry.has_expert("rust"));

        let removed = registry.remove("rust");
        assert!(removed.is_some());
        assert!(!registry.has_expert("rust"));
    }

    #[test]
    fn test_clear_registry() {
        let mut registry = ExpertRegistry::new();
        assert!(registry.count() > 0);

        registry.clear();
        assert_eq!(registry.count(), 0);
    }

    #[test]
    fn test_registry_builder() {
        let registry = ExpertRegistryBuilder::new()
            .with_defaults()
            .build();

        assert!(registry.has_expert("rust"));
    }

    #[test]
    fn test_registry_builder_custom() {
        let expert = Arc::new(RustExpert::new());
        let registry = ExpertRegistryBuilder::new()
            .with_expert(expert)
            .build();

        assert_eq!(registry.count(), 1);
        assert!(registry.has_expert("rust"));
    }

    #[test]
    fn test_find_compatible_experts() {
        let registry = ExpertRegistry::new();
        let compatible = registry.find_compatible("rust");
        assert!(!compatible.is_empty());
    }
}
