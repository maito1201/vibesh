use regex::Regex;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum ForbiddenPattern {
    Exact(String),
    Regex(Regex),
}

#[derive(Debug)]
pub struct Config {
    pub shell: String,
    pub forbidden: Vec<ForbiddenPattern>,
}

impl Config {
    pub fn load() -> Result<Self, String> {
        let config_path = Self::config_path()?;
        let content = fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        Self::parse(&content)
    }

    fn config_path() -> Result<PathBuf, String> {
        let home = dirs::home_dir()
            .ok_or_else(|| "Failed to get home directory".to_string())?;
        Ok(home.join(".vibeshrc"))
    }

    fn parse(content: &str) -> Result<Self, String> {
        let mut shell = None;
        let mut forbidden = Vec::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some(value) = line.strip_prefix("shell") {
                let value = value.trim().trim_start_matches('=').trim();
                shell = Some(value.to_string());
            } else if let Some(value) = line.strip_prefix("forbidden") {
                let value = value.trim().trim_start_matches('=').trim();
                let pattern = Self::parse_forbidden_pattern(value)?;
                forbidden.push(pattern);
            }
        }

        let shell = shell.ok_or_else(|| "Missing 'shell' configuration".to_string())?;
        Ok(Config { shell, forbidden })
    }

    fn parse_forbidden_pattern(value: &str) -> Result<ForbiddenPattern, String> {
        // Check for regexp('pattern') format
        if let Some(inner) = value.strip_prefix("regexp('").and_then(|s| s.strip_suffix("')")) {
            let regex = Regex::new(inner)
                .map_err(|e| format!("Invalid regex pattern: {}", e))?;
            return Ok(ForbiddenPattern::Regex(regex));
        }

        // Check for /pattern/ format
        if let Some(inner) = value.strip_prefix('/').and_then(|s| s.strip_suffix('/')) {
            let regex = Regex::new(inner)
                .map_err(|e| format!("Invalid regex pattern: {}", e))?;
            return Ok(ForbiddenPattern::Regex(regex));
        }

        // Exact match
        Ok(ForbiddenPattern::Exact(value.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config() {
        let content = r#"
shell = bash
forbidden = rm
forbidden = /^sudo.*/
forbidden = regexp('git push --force')
"#;
        let config = Config::parse(content).unwrap();
        assert_eq!(config.shell, "bash");
        assert_eq!(config.forbidden.len(), 3);
    }

    #[test]
    fn test_parse_exact_pattern() {
        let pattern = Config::parse_forbidden_pattern("rm").unwrap();
        match pattern {
            ForbiddenPattern::Exact(s) => assert_eq!(s, "rm"),
            _ => panic!("Expected Exact pattern"),
        }
    }

    #[test]
    fn test_parse_regex_pattern_slash() {
        let pattern = Config::parse_forbidden_pattern("/^sudo.*/").unwrap();
        match pattern {
            ForbiddenPattern::Regex(_) => (),
            _ => panic!("Expected Regex pattern"),
        }
    }

    #[test]
    fn test_parse_regex_pattern_function() {
        let pattern = Config::parse_forbidden_pattern("regexp('git.*')").unwrap();
        match pattern {
            ForbiddenPattern::Regex(_) => (),
            _ => panic!("Expected Regex pattern"),
        }
    }

    #[test]
    fn test_parse_missing_shell() {
        let content = "forbidden = rm";
        let result = Config::parse(content);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Missing 'shell'"));
    }

    #[test]
    fn test_parse_invalid_regex() {
        let result = Config::parse_forbidden_pattern("/[/");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_vibeshrc_example() {
        // Test that .vibeshrc.example can be parsed correctly
        let content = include_str!("../.vibeshrc.example");
        let config = Config::parse(content).unwrap();

        assert_eq!(config.shell, "bash");

        // Verify that multiple forbidden patterns are parsed
        assert!(config.forbidden.len() > 0);

        // Verify mix of exact match and regex patterns
        let has_exact = config.forbidden.iter().any(|p| matches!(p, ForbiddenPattern::Exact(_)));
        let has_regex = config.forbidden.iter().any(|p| matches!(p, ForbiddenPattern::Regex(_)));

        assert!(has_exact, "Should have at least one exact match pattern");
        assert!(has_regex, "Should have at least one regex pattern");
    }
}
