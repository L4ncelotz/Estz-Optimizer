use serde::Serialize;
use std::process::Command;

#[derive(Debug, Serialize, Clone)]
pub struct PsOutput {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

/// Execute a PowerShell command hidden (no window popup)
pub fn execute(script: &str) -> PsOutput {
    let output = Command::new("powershell.exe")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-WindowStyle",
            "Hidden",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            script,
        ])
        .output();

    match output {
        Ok(out) => PsOutput {
            success: out.status.success(),
            stdout: String::from_utf8_lossy(&out.stdout).trim().to_string(),
            stderr: String::from_utf8_lossy(&out.stderr).trim().to_string(),
            exit_code: out.status.code().unwrap_or(-1),
        },
        Err(e) => PsOutput {
            success: false,
            stdout: String::new(),
            stderr: format!("Failed to execute PowerShell: {}", e),
            exit_code: -1,
        },
    }
}

/// Execute a PowerShell script file
pub fn execute_script_file(path: &str) -> PsOutput {
    let script = format!(
        r#"& '{}'"#,
        path
    );
    execute(&script)
}

/// Get the current active power plan name
pub fn get_active_power_plan() -> String {
    let result = execute("(powercfg /getactivescheme) -replace '.*\\((.*)\\).*', '$1'");
    if result.success && !result.stdout.is_empty() {
        result.stdout
    } else {
        "Unknown".to_string()
    }
}

/// Check if a Windows service is running
pub fn is_service_running(service_name: &str) -> bool {
    let script = format!(
        "(Get-Service -Name '{}' -ErrorAction SilentlyContinue).Status -eq 'Running'",
        service_name
    );
    let result = execute(&script);
    result.stdout.to_lowercase().contains("true")
}

/// Get a list of installed Appx packages matching a pattern
pub fn get_appx_packages(pattern: &str) -> Vec<String> {
    let script = format!(
        "Get-AppxPackage -Name '*{}*' | Select-Object -ExpandProperty Name",
        pattern
    );
    let result = execute(&script);
    if result.success {
        result
            .stdout
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect()
    } else {
        Vec::new()
    }
}

/// Remove an Appx package
pub fn remove_appx_package(package_name: &str) -> PsOutput {
    let script = format!(
        "Get-AppxPackage -Name '{}' -AllUsers | Remove-AppxPackage -AllUsers -ErrorAction SilentlyContinue",
        package_name
    );
    execute(&script)
}
