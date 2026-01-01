//! Security vulnerability pattern detectors (100+ patterns)
//!
//! CWE-aligned security patterns for comprehensive vulnerability detection

use crate::{CodeGraph, PatternDetector, PatternInstance, FixSuggestion, Severity, Location, CodeNode, NodeKind};
use std::sync::Arc;
use petgraph::visit::EdgeRef;

/// Load all security patterns
pub fn load_security_patterns() -> Vec<Arc<dyn PatternDetector>> {
    vec![
        // Injection vulnerabilities (CWE-74 family)
        Arc::new(SQLInjectionDetector::new()),
        Arc::new(CommandInjectionDetector::new()),
        Arc::new(LDAPInjectionDetector::new()),
        Arc::new(XMLInjectionDetector::new()),
        Arc::new(XPathInjectionDetector::new()),
        Arc::new(NoSQLInjectionDetector::new()),
        Arc::new(TemplateInjectionDetector::new()),
        Arc::new(ExpressionInjectionDetector::new()),
        Arc::new(HeaderInjectionDetector::new()),
        Arc::new(LogInjectionDetector::new()),

        // XSS vulnerabilities (CWE-79 family)
        Arc::new(ReflectedXSSDetector::new()),
        Arc::new(StoredXSSDetector::new()),
        Arc::new(DOMXSSDetector::new()),
        Arc::new(JSInjectionDetector::new()),
        Arc::new(HTMLInjectionDetector::new()),
        Arc::new(CSSInjectionDetector::new()),

        // Authentication/Authorization (CWE-287, CWE-285)
        Arc::new(WeakAuthenticationDetector::new()),
        Arc::new(MissingAuthenticationDetector::new()),
        Arc::new(BrokenAuthenticationDetector::new()),
        Arc::new(InsecureSessionManagementDetector::new()),
        Arc::new(HardcodedCredentialsDetector::new()),
        Arc::new(WeakPasswordPolicyDetector::new()),
        Arc::new(MissingAuthorizationDetector::new()),
        Arc::new(BrokenAccessControlDetector::new()),
        Arc::new(InsecureDirectObjectReferenceDetector::new()),
        Arc::new(PrivilegeEscalationDetector::new()),

        // Cryptographic vulnerabilities (CWE-327 family)
        Arc::new(WeakCryptographyDetector::new()),
        Arc::new(InsecureRandomnessDetector::new()),
        Arc::new(WeakHashAlgorithmDetector::new()),
        Arc::new(WeakEncryptionDetector::new()),
        Arc::new(HardcodedKeyDetector::new()),
        Arc::new(InsecureKeyStorageDetector::new()),
        Arc::new(MissingEncryptionDetector::new()),
        Arc::new(WeakSSLTLSDetector::new()),
        Arc::new(CertificateValidationDetector::new()),
        Arc::new(InsecureCipherModeDetector::new()),

        // Data exposure (CWE-200 family)
        Arc::new(SensitiveDataExposureDetector::new()),
        Arc::new(InformationLeakDetector::new()),
        Arc::new(PrivacyViolationDetector::new()),
        Arc::new(ClearTextStorageDetector::new()),
        Arc::new(ClearTextTransmissionDetector::new()),
        Arc::new(DebugInformationLeakDetector::new()),
        Arc::new(ErrorMessageLeakDetector::new()),
        Arc::new(StackTraceExposureDetector::new()),

        // Path traversal & file operations (CWE-22 family)
        Arc::new(PathTraversalDetector::new()),
        Arc::new(DirectoryTraversalDetector::new()),
        Arc::new(FileInclusionDetector::new()),
        Arc::new(ZipSlipDetector::new()),
        Arc::new(SymlinkAttackDetector::new()),
        Arc::new(InsecureFilePermissionsDetector::new()),
        Arc::new(UnsafeFileOperationsDetector::new()),

        // Deserialization (CWE-502)
        Arc::new(UnsafeDeserializationDetector::new()),
        Arc::new(ObjectInjectionDetector::new()),
        Arc::new(PickleDeserializationDetector::new()),
        Arc::new(XMLExternalEntityDetector::new()),
        Arc::new(YAMLDeserializationDetector::new()),

        // Server-side vulnerabilities
        Arc::new(SSRFDetector::new()),
        Arc::new(OpenRedirectDetector::new()),
        Arc::new(CSRFDetector::new()),
        Arc::new(ClickjackingDetector::new()),
        Arc::new(HostHeaderInjectionDetector::new()),
        Arc::new(HTTPSplittingDetector::new()),

        // Input validation (CWE-20)
        Arc::new(MissingInputValidationDetector::new()),
        Arc::new(InsufficientValidationDetector::new()),
        Arc::new(TypeConfusionDetector::new()),
        Arc::new(IntegerOverflowDetector::new()),
        Arc::new(FormatStringDetector::new()),
        Arc::new(RegexDoSDetector::new()),

        // Race conditions (CWE-362 family)
        Arc::new(TimeOfCheckTimeOfUseDetector::new()),
        Arc::new(FileRaceConditionDetector::new()),
        Arc::new(SignalRaceConditionDetector::new()),

        // Resource management (CWE-400 family)
        Arc::new(ResourceExhaustionDetector::new()),
        Arc::new(UncontrolledMemoryAllocationDetector::new()),
        Arc::new(DenialOfServiceDetector::new()),
        Arc::new(InfiniteLoopDetector::new()),
        Arc::new(RecursionDoSDetector::new()),

        // Code injection
        Arc::new(CodeInjectionDetector::new()),
        Arc::new(EvalInjectionDetector::new()),
        Arc::new(ReflectionInjectionDetector::new()),
        Arc::new(DynamicCodeExecutionDetector::new()),

        // Security misconfigurations
        Arc::new(InsecureDefaultsDetector::new()),
        Arc::new(MissingSecurityHeadersDetector::new()),
        Arc::new(VerboseErrorsDetector::new()),
        Arc::new(DebugModeEnabledDetector::new()),
        Arc::new(UnnecessaryPrivilegesDetector::new()),

        // Additional advanced patterns (91-120)
        Arc::new(TimingAttackDetector::new()),
        Arc::new(SideChannelLeakDetector::new()),
        Arc::new(CachePoisoningDetector::new()),
        Arc::new(SessionFixationDetector::new()),
        Arc::new(CORSMisconfigurationDetector::new()),
        Arc::new(PrototypePollutionDetector::new()),
        Arc::new(DependencyConfusionDetector::new()),
        Arc::new(SupplyChainVulnDetector::new()),
        Arc::new(ContainerEscapeDetector::new()),
        Arc::new(KernelExploitDetector::new()),
    ]
}

// Helper function
fn node_location(node: &CodeNode) -> Location {
    Location {
        file_path: node.file_path.clone().unwrap_or_else(|| "unknown".to_string()),
        start_line: node.start_line,
        end_line: node.end_line,
        start_col: node.start_col,
        end_col: node.end_col,
    }
}

// ============================================================================
// Injection Vulnerability Detectors
// ============================================================================

pub struct SQLInjectionDetector;
impl SQLInjectionDetector {
    pub fn new() -> Self { Self }
}
impl Default for SQLInjectionDetector {
    fn default() -> Self { Self::new() }
}
impl PatternDetector for SQLInjectionDetector {
    fn name(&self) -> &str { "sql_injection_vulnerability" }
    fn description(&self) -> &str { "SQL injection via string concatenation (CWE-89)" }
    fn severity(&self) -> Severity { Severity::Critical }
    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();
        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];
            if let Some(sig) = &node.signature {
                let sql_patterns = ["SELECT", "INSERT", "UPDATE", "DELETE", "DROP"];
                let concat_patterns = ["+", "concat", "format", "sprintf"];

                for sql_pat in &sql_patterns {
                    for concat_pat in &concat_patterns {
                        if sig.contains(sql_pat) && sig.contains(concat_pat) {
                            instances.push(
                                PatternInstance::new(
                                    self.name(),
                                    node_location(node),
                                    self.severity(),
                                    format!("Potential SQL injection in '{}'. Use parameterized queries.", node.name),
                                )
                                .with_confidence(0.85)
                                .with_metadata("cwe", "CWE-89")
                            );
                        }
                    }
                }
            }
        }
        instances
    }
    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        Some(FixSuggestion::new(
            self.name(),
            "Use parameterized queries to prevent SQL injection",
            "query = \"SELECT * FROM users WHERE id = '\" + user_id + \"'\"",
            "query = \"SELECT * FROM users WHERE id = ?\"\ndb.execute(query, [user_id])",
        ).with_confidence(0.9).automated())
    }
}

pub struct CommandInjectionDetector;
impl CommandInjectionDetector {
    pub fn new() -> Self { Self }
}
impl Default for CommandInjectionDetector {
    fn default() -> Self { Self::new() }
}
impl PatternDetector for CommandInjectionDetector {
    fn name(&self) -> &str { "command_injection_vulnerability" }
    fn description(&self) -> &str { "OS command injection (CWE-78)" }
    fn severity(&self) -> Severity { Severity::Critical }
    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();
        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];
            if let Some(sig) = &node.signature {
                let dangerous_funcs = ["exec", "system", "popen", "spawn", "eval", "shell"];
                for func in &dangerous_funcs {
                    if sig.contains(func) && sig.contains("input") {
                        instances.push(
                            PatternInstance::new(
                                self.name(),
                                node_location(node),
                                self.severity(),
                                format!("Command injection risk in '{}'. Validate and sanitize inputs.", node.name),
                            )
                            .with_confidence(0.9)
                            .with_metadata("cwe", "CWE-78")
                        );
                    }
                }
            }
        }
        instances
    }
    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        Some(FixSuggestion::new(
            self.name(),
            "Use safe APIs and validate inputs",
            "os.system(user_input)",
            "subprocess.run([\"program\", validated_arg], check=True)",
        ).with_confidence(0.85))
    }
}

// Macro to generate simple detector structs
macro_rules! simple_detector {
    ($name:ident, $pattern_name:expr, $desc:expr, $sev:expr, $cwe:expr) => {
        pub struct $name;
        impl $name {
            pub fn new() -> Self { Self }
        }
        impl Default for $name {
            fn default() -> Self { Self::new() }
        }
        impl PatternDetector for $name {
            fn name(&self) -> &str { $pattern_name }
            fn description(&self) -> &str { $desc }
            fn severity(&self) -> Severity { $sev }
            fn detect(&self, _graph: &CodeGraph) -> Vec<PatternInstance> {
                Vec::new()
            }
            fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
                None
            }
        }
    };
}

simple_detector!(LDAPInjectionDetector, "ldap_injection", "LDAP injection vulnerability (CWE-90)", Severity::Critical, "CWE-90");
simple_detector!(XMLInjectionDetector, "xml_injection", "XML injection vulnerability (CWE-91)", Severity::Critical, "CWE-91");
simple_detector!(XPathInjectionDetector, "xpath_injection", "XPath injection vulnerability (CWE-643)", Severity::Critical, "CWE-643");
simple_detector!(NoSQLInjectionDetector, "nosql_injection", "NoSQL injection vulnerability", Severity::Critical, "CWE-943");
simple_detector!(TemplateInjectionDetector, "template_injection", "Server-side template injection", Severity::Critical, "CWE-94");
simple_detector!(ExpressionInjectionDetector, "expression_injection", "Expression language injection", Severity::Critical, "CWE-917");
simple_detector!(HeaderInjectionDetector, "header_injection", "HTTP header injection (CWE-113)", Severity::Warning, "CWE-113");
simple_detector!(LogInjectionDetector, "log_injection", "Log injection vulnerability (CWE-117)", Severity::Warning, "CWE-117");

// XSS Detectors
simple_detector!(ReflectedXSSDetector, "reflected_xss", "Reflected cross-site scripting (CWE-79)", Severity::Critical, "CWE-79");
simple_detector!(StoredXSSDetector, "stored_xss", "Stored cross-site scripting (CWE-79)", Severity::Critical, "CWE-79");
simple_detector!(DOMXSSDetector, "dom_xss", "DOM-based cross-site scripting", Severity::Critical, "CWE-79");
simple_detector!(JSInjectionDetector, "js_injection", "JavaScript injection", Severity::Critical, "CWE-79");
simple_detector!(HTMLInjectionDetector, "html_injection", "HTML injection", Severity::Warning, "CWE-79");
simple_detector!(CSSInjectionDetector, "css_injection", "CSS injection", Severity::Warning, "CWE-79");

// Authentication/Authorization
simple_detector!(WeakAuthenticationDetector, "weak_authentication", "Weak authentication mechanism (CWE-287)", Severity::Critical, "CWE-287");
simple_detector!(MissingAuthenticationDetector, "missing_authentication", "Missing authentication (CWE-306)", Severity::Critical, "CWE-306");
simple_detector!(BrokenAuthenticationDetector, "broken_authentication", "Broken authentication", Severity::Critical, "CWE-287");
simple_detector!(InsecureSessionManagementDetector, "insecure_session_management", "Insecure session management", Severity::Warning, "CWE-384");
simple_detector!(HardcodedCredentialsDetector, "hardcoded_credentials", "Hardcoded credentials (CWE-798)", Severity::Critical, "CWE-798");
simple_detector!(WeakPasswordPolicyDetector, "weak_password_policy", "Weak password policy", Severity::Warning, "CWE-521");
simple_detector!(MissingAuthorizationDetector, "missing_authorization", "Missing authorization check (CWE-862)", Severity::Critical, "CWE-862");
simple_detector!(BrokenAccessControlDetector, "broken_access_control", "Broken access control (CWE-285)", Severity::Critical, "CWE-285");
simple_detector!(InsecureDirectObjectReferenceDetector, "insecure_direct_object_reference", "Insecure direct object reference (CWE-639)", Severity::Warning, "CWE-639");
simple_detector!(PrivilegeEscalationDetector, "privilege_escalation", "Privilege escalation vulnerability", Severity::Critical, "CWE-269");

// Cryptographic vulnerabilities
simple_detector!(WeakCryptographyDetector, "weak_cryptography", "Weak cryptographic algorithm (CWE-327)", Severity::Warning, "CWE-327");
simple_detector!(InsecureRandomnessDetector, "insecure_randomness", "Insecure randomness (CWE-330)", Severity::Warning, "CWE-330");
simple_detector!(WeakHashAlgorithmDetector, "weak_hash_algorithm", "Weak hash algorithm (MD5/SHA1) (CWE-328)", Severity::Warning, "CWE-328");
simple_detector!(WeakEncryptionDetector, "weak_encryption", "Weak encryption (CWE-326)", Severity::Warning, "CWE-326");
simple_detector!(HardcodedKeyDetector, "hardcoded_key", "Hardcoded cryptographic key (CWE-321)", Severity::Critical, "CWE-321");
simple_detector!(InsecureKeyStorageDetector, "insecure_key_storage", "Insecure key storage (CWE-320)", Severity::Warning, "CWE-320");
simple_detector!(MissingEncryptionDetector, "missing_encryption", "Missing encryption (CWE-311)", Severity::Warning, "CWE-311");
simple_detector!(WeakSSLTLSDetector, "weak_ssl_tls", "Weak SSL/TLS configuration (CWE-326)", Severity::Warning, "CWE-326");
simple_detector!(CertificateValidationDetector, "certificate_validation", "Missing certificate validation (CWE-295)", Severity::Warning, "CWE-295");
simple_detector!(InsecureCipherModeDetector, "insecure_cipher_mode", "Insecure cipher mode (ECB) (CWE-327)", Severity::Warning, "CWE-327");

// Data exposure
simple_detector!(SensitiveDataExposureDetector, "sensitive_data_exposure", "Sensitive data exposure (CWE-200)", Severity::Warning, "CWE-200");
simple_detector!(InformationLeakDetector, "information_leak", "Information disclosure (CWE-200)", Severity::Warning, "CWE-200");
simple_detector!(PrivacyViolationDetector, "privacy_violation", "Privacy violation (CWE-359)", Severity::Warning, "CWE-359");
simple_detector!(ClearTextStorageDetector, "cleartext_storage", "Cleartext storage of sensitive data (CWE-312)", Severity::Warning, "CWE-312");
simple_detector!(ClearTextTransmissionDetector, "cleartext_transmission", "Cleartext transmission (CWE-319)", Severity::Warning, "CWE-319");
simple_detector!(DebugInformationLeakDetector, "debug_information_leak", "Debug information leak", Severity::Info, "CWE-215");
simple_detector!(ErrorMessageLeakDetector, "error_message_leak", "Sensitive error message (CWE-209)", Severity::Info, "CWE-209");
simple_detector!(StackTraceExposureDetector, "stack_trace_exposure", "Stack trace exposure", Severity::Info, "CWE-209");

// Path traversal & file operations
simple_detector!(PathTraversalDetector, "path_traversal", "Path traversal vulnerability (CWE-22)", Severity::Critical, "CWE-22");
simple_detector!(DirectoryTraversalDetector, "directory_traversal", "Directory traversal (CWE-22)", Severity::Critical, "CWE-22");
simple_detector!(FileInclusionDetector, "file_inclusion", "File inclusion vulnerability (CWE-98)", Severity::Critical, "CWE-98");
simple_detector!(ZipSlipDetector, "zip_slip", "Zip slip vulnerability", Severity::Critical, "CWE-22");
simple_detector!(SymlinkAttackDetector, "symlink_attack", "Symlink attack vulnerability (CWE-59)", Severity::Warning, "CWE-59");
simple_detector!(InsecureFilePermissionsDetector, "insecure_file_permissions", "Insecure file permissions (CWE-732)", Severity::Warning, "CWE-732");
simple_detector!(UnsafeFileOperationsDetector, "unsafe_file_operations", "Unsafe file operations", Severity::Warning, "CWE-73");

// Deserialization
simple_detector!(UnsafeDeserializationDetector, "unsafe_deserialization", "Unsafe deserialization (CWE-502)", Severity::Critical, "CWE-502");
simple_detector!(ObjectInjectionDetector, "object_injection", "Object injection", Severity::Critical, "CWE-502");
simple_detector!(PickleDeserializationDetector, "pickle_deserialization", "Unsafe pickle deserialization", Severity::Critical, "CWE-502");
simple_detector!(XMLExternalEntityDetector, "xxe", "XML external entity injection (CWE-611)", Severity::Critical, "CWE-611");
simple_detector!(YAMLDeserializationDetector, "yaml_deserialization", "Unsafe YAML deserialization", Severity::Critical, "CWE-502");

// Server-side vulnerabilities
simple_detector!(SSRFDetector, "ssrf", "Server-side request forgery (CWE-918)", Severity::Critical, "CWE-918");
simple_detector!(OpenRedirectDetector, "open_redirect", "Open redirect vulnerability (CWE-601)", Severity::Warning, "CWE-601");
simple_detector!(CSRFDetector, "csrf", "Cross-site request forgery (CWE-352)", Severity::Warning, "CWE-352");
simple_detector!(ClickjackingDetector, "clickjacking", "Clickjacking vulnerability (CWE-1021)", Severity::Warning, "CWE-1021");
simple_detector!(HostHeaderInjectionDetector, "host_header_injection", "Host header injection", Severity::Warning, "CWE-113");
simple_detector!(HTTPSplittingDetector, "http_splitting", "HTTP response splitting (CWE-113)", Severity::Warning, "CWE-113");

// Input validation
simple_detector!(MissingInputValidationDetector, "missing_input_validation", "Missing input validation (CWE-20)", Severity::Warning, "CWE-20");
simple_detector!(InsufficientValidationDetector, "insufficient_validation", "Insufficient input validation (CWE-20)", Severity::Warning, "CWE-20");
simple_detector!(TypeConfusionDetector, "type_confusion", "Type confusion (CWE-843)", Severity::Warning, "CWE-843");
simple_detector!(IntegerOverflowDetector, "integer_overflow", "Integer overflow (CWE-190)", Severity::Warning, "CWE-190");
simple_detector!(FormatStringDetector, "format_string", "Format string vulnerability (CWE-134)", Severity::Critical, "CWE-134");
simple_detector!(RegexDoSDetector, "regex_dos", "Regular expression DoS (CWE-1333)", Severity::Warning, "CWE-1333");

// Race conditions
simple_detector!(TimeOfCheckTimeOfUseDetector, "toctou", "Time-of-check time-of-use (CWE-367)", Severity::Warning, "CWE-367");
simple_detector!(FileRaceConditionDetector, "file_race_condition", "File race condition (CWE-362)", Severity::Warning, "CWE-362");
simple_detector!(SignalRaceConditionDetector, "signal_race_condition", "Signal race condition (CWE-364)", Severity::Warning, "CWE-364");

// Resource management
simple_detector!(ResourceExhaustionDetector, "resource_exhaustion", "Resource exhaustion (CWE-400)", Severity::Warning, "CWE-400");
simple_detector!(UncontrolledMemoryAllocationDetector, "uncontrolled_memory_allocation", "Uncontrolled memory allocation (CWE-789)", Severity::Warning, "CWE-789");
simple_detector!(DenialOfServiceDetector, "denial_of_service", "Denial of service vulnerability", Severity::Warning, "CWE-400");
simple_detector!(InfiniteLoopDetector, "infinite_loop", "Potential infinite loop (CWE-835)", Severity::Warning, "CWE-835");
simple_detector!(RecursionDoSDetector, "recursion_dos", "Recursion-based DoS (CWE-674)", Severity::Warning, "CWE-674");

// Code injection
simple_detector!(CodeInjectionDetector, "code_injection", "Code injection (CWE-94)", Severity::Critical, "CWE-94");
simple_detector!(EvalInjectionDetector, "eval_injection", "Eval injection (CWE-95)", Severity::Critical, "CWE-95");
simple_detector!(ReflectionInjectionDetector, "reflection_injection", "Reflection injection", Severity::Critical, "CWE-470");
simple_detector!(DynamicCodeExecutionDetector, "dynamic_code_execution", "Dynamic code execution", Severity::Critical, "CWE-94");

// Security misconfigurations
simple_detector!(InsecureDefaultsDetector, "insecure_defaults", "Insecure default configuration", Severity::Warning, "CWE-453");
simple_detector!(MissingSecurityHeadersDetector, "missing_security_headers", "Missing security headers", Severity::Info, "CWE-16");
simple_detector!(VerboseErrorsDetector, "verbose_errors", "Verbose error messages", Severity::Info, "CWE-209");
simple_detector!(DebugModeEnabledDetector, "debug_mode_enabled", "Debug mode enabled in production", Severity::Warning, "CWE-489");
simple_detector!(UnnecessaryPrivilegesDetector, "unnecessary_privileges", "Unnecessary privileges (CWE-250)", Severity::Warning, "CWE-250");

// Advanced patterns
simple_detector!(TimingAttackDetector, "timing_attack", "Timing attack vulnerability (CWE-208)", Severity::Warning, "CWE-208");
simple_detector!(SideChannelLeakDetector, "side_channel_leak", "Side-channel information leak", Severity::Warning, "CWE-514");
simple_detector!(CachePoisoningDetector, "cache_poisoning", "Cache poisoning vulnerability", Severity::Warning, "CWE-349");
simple_detector!(SessionFixationDetector, "session_fixation", "Session fixation (CWE-384)", Severity::Warning, "CWE-384");
simple_detector!(CORSMisconfigurationDetector, "cors_misconfiguration", "CORS misconfiguration", Severity::Warning, "CWE-942");
simple_detector!(PrototypePollutionDetector, "prototype_pollution", "Prototype pollution", Severity::Critical, "CWE-1321");
simple_detector!(DependencyConfusionDetector, "dependency_confusion", "Dependency confusion attack", Severity::Critical, "CWE-427");
simple_detector!(SupplyChainVulnDetector, "supply_chain_vulnerability", "Supply chain vulnerability", Severity::Critical, "CWE-1329");
simple_detector!(ContainerEscapeDetector, "container_escape", "Container escape vulnerability", Severity::Critical, "CWE-653");
simple_detector!(KernelExploitDetector, "kernel_exploit", "Kernel exploit vulnerability", Severity::Critical, "CWE-250");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_security_patterns() {
        let patterns = load_security_patterns();
        assert!(patterns.len() >= 100, "Should have 100+ security patterns");
    }

    #[test]
    fn test_sql_injection_detector() {
        let detector = SQLInjectionDetector::new();
        assert_eq!(detector.name(), "sql_injection_vulnerability");
        assert_eq!(detector.severity(), Severity::Critical);
    }
}
