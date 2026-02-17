use crate::config::{Config, ForbiddenPattern};

pub struct CommandFilter<'a> {
    config: &'a Config,
}

impl<'a> CommandFilter<'a> {
    pub fn new(config: &'a Config) -> Self {
        CommandFilter { config }
    }

    pub fn is_allowed(&self, input: &str) -> bool {
        !self.is_forbidden(input)
    }

    fn extract_command(input: &str) -> Option<String> {
        input
            .trim()
            .split_whitespace()
            .next()
            .map(|s| s.to_string())
    }

    fn is_forbidden(&self, input: &str) -> bool {
        let Some(command) = Self::extract_command(input) else {
            return false;
        };
        self.config.forbidden.iter().any(|pattern| {
            match pattern {
                ForbiddenPattern::Exact(s) => s == &command,
                ForbiddenPattern::Regex(re) => re.is_match(input),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ForbiddenPattern;
    use regex::Regex;

    fn create_test_config(forbidden: Vec<ForbiddenPattern>) -> Config {
        Config {
            shell: "bash".to_string(),
            forbidden,
        }
    }

    #[test]
    fn test_extract_command() {
        assert_eq!(CommandFilter::extract_command("ls -la"), Some("ls".to_string()));
        assert_eq!(CommandFilter::extract_command("rm file.txt"), Some("rm".to_string()));
        assert_eq!(CommandFilter::extract_command("  echo hello  "), Some("echo".to_string()));
        assert_eq!(CommandFilter::extract_command(""), None);
        assert_eq!(CommandFilter::extract_command("   "), None);
        assert_eq!(CommandFilter::extract_command("\t"), None);
    }

    #[test]
    fn test_exact_match_forbidden() {
        let config = create_test_config(vec![
            ForbiddenPattern::Exact("rm".to_string()),
        ]);
        let filter = CommandFilter::new(&config);

        assert!(!filter.is_allowed("rm file.txt"));
        assert!(!filter.is_allowed("rm"));
        assert!(filter.is_allowed("ls"));
    }

    #[test]
    fn test_regex_match_forbidden() {
        let config = create_test_config(vec![
            ForbiddenPattern::Regex(Regex::new("^sudo.*").unwrap()),
        ]);
        let filter = CommandFilter::new(&config);

        assert!(!filter.is_allowed("sudo rm -rf"));
        assert!(!filter.is_allowed("sudo apt install"));
        assert!(filter.is_allowed("ls"));
    }

    #[test]
    fn test_multiple_patterns() {
        let config = create_test_config(vec![
            ForbiddenPattern::Exact("rm".to_string()),
            ForbiddenPattern::Regex(Regex::new("^sudo.*").unwrap()),
        ]);
        let filter = CommandFilter::new(&config);

        assert!(!filter.is_allowed("rm file.txt"));
        assert!(!filter.is_allowed("sudo rm"));
        assert!(filter.is_allowed("ls"));
        assert!(filter.is_allowed("cat file.txt"));
    }

    #[test]
    fn test_command_with_arguments_ignored() {
        let config = create_test_config(vec![
            ForbiddenPattern::Exact("rm".to_string()),
        ]);
        let filter = CommandFilter::new(&config);

        assert!(!filter.is_allowed("rm -rf /"));
        assert!(!filter.is_allowed("rm ./something"));
    }

    #[test]
    fn test_regex_with_arguments() {
        let config = create_test_config(vec![
            ForbiddenPattern::Regex(Regex::new("git.*--force").unwrap()),
        ]);
        let filter = CommandFilter::new(&config);

        assert!(!filter.is_allowed("git push --force"));
        assert!(!filter.is_allowed("git commit --force"));
        assert!(filter.is_allowed("git push"));
        assert!(filter.is_allowed("git commit"));
    }

    #[test]
    fn test_empty_input() {
        let config = create_test_config(vec![
            ForbiddenPattern::Exact("rm".to_string()),
        ]);
        let filter = CommandFilter::new(&config);

        assert!(filter.is_allowed(""));
        assert!(filter.is_allowed("   "));
        assert!(filter.is_allowed("\t"));
    }
}
