use serde::Serialize;
use std::process::Command;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Debug, Serialize, Clone)]
pub struct PsOutput {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

/// Execute a PowerShell command hidden (no window popup)
pub fn execute(script: &str) -> PsOutput {
    #[cfg(target_os = "windows")]
    let mut cmd = Command::new("powershell.exe");
    #[cfg(target_os = "windows")]
    cmd.creation_flags(CREATE_NO_WINDOW);
    
    #[cfg(not(target_os = "windows"))]
    let mut cmd = Command::new("powershell.exe");

    let output = cmd
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
#[allow(dead_code)]
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

/// Sanitize string for PowerShell single-quote injection
fn escape_ps_string(input: &str) -> String {
    input.replace("'", "''")
}

/// Check if a Windows service is running
pub fn is_service_running(service_name: &str) -> bool {
    let safe_name = escape_ps_string(service_name);
    let script = format!(
        "(Get-Service -Name '{}' -ErrorAction SilentlyContinue).Status -eq 'Running'",
        safe_name
    );
    let result = execute(&script);
    result.stdout.to_lowercase().contains("true")
}

/// Get a list of installed Appx packages matching a pattern
pub fn get_appx_packages(pattern: &str) -> Vec<String> {
    let safe_pattern = escape_ps_string(pattern);
    let script = format!(
        "Get-AppxPackage -Name '*{}*' | Select-Object -ExpandProperty Name",
        safe_pattern
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
    let safe_name = escape_ps_string(package_name);
    let script = format!(
        "Get-AppxPackage -Name '{}' -AllUsers | Remove-AppxPackage -AllUsers -ErrorAction SilentlyContinue",
        safe_name
    );
    execute(&script)
}
