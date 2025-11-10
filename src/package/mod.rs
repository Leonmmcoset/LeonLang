use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub struct PackageManager {
    loaded_packages: HashMap<String, bool>,
    require_path: PathBuf,
}

impl PackageManager {
    pub fn new() -> Self {
        Self {
            loaded_packages: HashMap::new(),
            require_path: PathBuf::from("./"),
        }
    }
    
    pub fn set_require_path(&mut self, path: &str) {
        self.require_path = PathBuf::from(path);
    }
    
    pub fn load_package(&mut self, package_name: &str) -> Result<String, String> {
        // Check if already loaded
        if let Some(_) = self.loaded_packages.get(package_name) {
            return Ok("".to_string());
        }
        
        // For built-in libraries, return success and mark as loaded
        if package_name == "basic" || package_name == "request" {
            self.loaded_packages.insert(package_name.to_string(), true);
            return Ok("".to_string());
        }
        
        // Try to load custom modules from require_path
        let file_path = self.require_path.join(format!("{}.leon", package_name));
        
        if file_path.exists() {
            match fs::read_to_string(&file_path) {
                Ok(content) => {
                    self.loaded_packages.insert(package_name.to_string(), true);
                    Ok(content)
                },
                Err(e) => Err(format!("Failed to read module file: {}", e)),
            }
        } else {
            Err(format!("Module not found: {}", package_name))
        }
    }
    
    pub fn is_package_loaded(&self, package_name: &str) -> bool {
        self.loaded_packages.contains_key(package_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_builtin_packages() {
        let mut pm = PackageManager::new();
        assert!(pm.load_package("basic").is_ok());
        assert!(pm.is_package_loaded("basic"));
    }
}