use serde::{Deserialize, Serialize};
use std::io;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ModDescriptor {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<String>,
    pub tags: Vec<String>,
    pub supported_version: String,
    pub local_path: String,
    pub remote_file_id: String,
}

impl ModDescriptor {
    pub fn new() -> Self {
        ModDescriptor {
            name: String::new(),
            version: String::new(),
            dependencies: Vec::new(),
            tags: Vec::new(),
            supported_version: String::new(),
            local_path: String::new(),
            remote_file_id: String::new(),
        }
    }



}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_mod_info() -> ModDescriptor {
        ModDescriptor::new(
            "test_mod".to_string(),
            "1.0.0".to_string(),
            vec!["dep1".to_string(), "dep2".to_string()],
            vec!["tag1".to_string(), "tag2".to_string()],
            "1.18".to_string(),
            "/path/to/mod".to_string(),
            "12345".to_string(),
        )
    }

    #[test]
    fn test_new_mod_info() {
        let mod_info = create_test_mod_info();
        assert_eq!(mod_info.name, "test_mod");
        assert_eq!(mod_info.version, "1.0.0");
        assert_eq!(mod_info.dependencies, vec!["dep1", "dep2"]);
        assert_eq!(mod_info.tags, vec!["tag1", "tag2"]);
        assert_eq!(mod_info.supported_version, "1.18");
        assert_eq!(mod_info.local_path, "/path/to/mod");
        assert_eq!(mod_info.remote_file_id, "12345");
    }

    #[test]
    fn test_has_dependency() {
        let mod_info = create_test_mod_info();
        assert!(mod_info.has_dependency("dep1"));
        assert!(!mod_info.has_dependency("dep3"));
    }

    #[test]
    fn test_has_tag() {
        let mod_info = create_test_mod_info();
        assert!(mod_info.has_tag("tag1"));
        assert!(!mod_info.has_tag("tag3"));
    }

    #[test]
    fn test_add_dependency() {
        let mut mod_info = create_test_mod_info();
        mod_info.add_dependency("dep3".to_string());
        assert!(mod_info.has_dependency("dep3"));

        // Test duplicate dependency
        mod_info.add_dependency("dep3".to_string());
        assert_eq!(
            mod_info.dependencies.iter().filter(|&d| d == "dep3").count(),
            1
        );
    }

    #[test]
    fn test_add_tag() {
        let mut mod_info = create_test_mod_info();
        mod_info.add_tag("tag3".to_string());
        assert!(mod_info.has_tag("tag3"));

        // Test duplicate tag
        mod_info.add_tag("tag3".to_string());
        assert_eq!(
            mod_info.tags.iter().filter(|&t| t == "tag3").count(),
            1
        );
    }

    #[test]
    fn test_serialization() -> Result<(), io::Error> {
        let mod_info = create_test_mod_info();

        // Test serialization to JSON string
        let json = mod_info.to_json_string()?;

        // Test deserialization from JSON string
        let loaded_mod_info = ModDescriptor::from_json_string(&json)?;

        assert_eq!(mod_info, loaded_mod_info);
        Ok(())
    }
}