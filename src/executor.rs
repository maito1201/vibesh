use std::process::{Command, Stdio};

pub struct ShellExecutor {
    shell: String,
}

impl ShellExecutor {
    pub fn new(shell: String) -> Self {
        ShellExecutor { shell }
    }

    pub fn execute(&self, command: &str) -> Result<(), String> {
        let output = Command::new(&self.shell)
            .arg("-c")
            .arg(command)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .map_err(|e| format!("Failed to execute command: {}", e))?;

        if !output.status.success() {
            return Err(format!("Command failed with status: {}", output.status));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_simple_command() {
        let executor = ShellExecutor::new("bash".to_string());
        let result = executor.execute("echo test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_failing_command() {
        let executor = ShellExecutor::new("bash".to_string());
        let result = executor.execute("false");
        assert!(result.is_err());
    }

    #[test]
    fn test_execute_with_zsh() {
        let executor = ShellExecutor::new("zsh".to_string());
        let result = executor.execute("echo test");
        // May fail if zsh is not installed, but should handle gracefully
        if result.is_err() {
            let err = result.unwrap_err();
            assert!(err.contains("Failed to execute") || err.contains("status"));
        }
    }
}
