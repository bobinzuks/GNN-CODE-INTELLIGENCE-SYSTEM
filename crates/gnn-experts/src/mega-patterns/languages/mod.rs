//! Language-specific pattern collections (20+ languages)

pub mod rust;
pub mod python;
pub mod javascript;
pub mod typescript;
pub mod java;
pub mod cpp;
pub mod c;
pub mod go;
pub mod csharp;
pub mod kotlin;
pub mod swift;
pub mod ruby;
pub mod php;
pub mod scala;
pub mod elixir;
pub mod haskell;
pub mod ocaml;
pub mod erlang;
pub mod clojure;
pub mod lua;
pub mod r;
pub mod julia;
pub mod dart;
pub mod nim;

use crate::{PatternDetector, CodeGraph, PatternInstance, FixSuggestion, Severity};
use std::sync::Arc;

/// Language-specific pattern registry
pub struct LanguagePatternRegistry {
    languages: Vec<&'static str>,
}

impl LanguagePatternRegistry {
    pub fn new() -> Self {
        Self {
            languages: vec![
                "rust", "python", "javascript", "typescript", "java",
                "cpp", "c", "go", "csharp", "kotlin", "swift", "ruby",
                "php", "scala", "elixir", "haskell", "ocaml", "erlang",
                "clojure", "lua", "r", "julia", "dart", "nim",
            ],
        }
    }

    pub fn get_patterns(&self, language: &str) -> Vec<Arc<dyn PatternDetector>> {
        match language {
            "rust" => rust::get_rust_patterns(),
            "python" => python::get_python_patterns(),
            "javascript" => javascript::get_javascript_patterns(),
            "typescript" => typescript::get_typescript_patterns(),
            "java" => java::get_java_patterns(),
            "cpp" => cpp::get_cpp_patterns(),
            "c" => c::get_c_patterns(),
            "go" => go::get_go_patterns(),
            "csharp" => csharp::get_csharp_patterns(),
            "kotlin" => kotlin::get_kotlin_patterns(),
            "swift" => swift::get_swift_patterns(),
            "ruby" => ruby::get_ruby_patterns(),
            "php" => php::get_php_patterns(),
            "scala" => scala::get_scala_patterns(),
            "elixir" => elixir::get_elixir_patterns(),
            "haskell" => haskell::get_haskell_patterns(),
            "ocaml" => ocaml::get_ocaml_patterns(),
            "erlang" => erlang::get_erlang_patterns(),
            "clojure" => clojure::get_clojure_patterns(),
            "lua" => lua::get_lua_patterns(),
            "r" => r::get_r_patterns(),
            "julia" => julia::get_julia_patterns(),
            "dart" => dart::get_dart_patterns(),
            "nim" => nim::get_nim_patterns(),
            _ => Vec::new(),
        }
    }

    pub fn supported_languages(&self) -> &[&'static str] {
        &self.languages
    }
}

impl Default for LanguagePatternRegistry {
    fn default() -> Self {
        Self::new()
    }
}
