use serde::Serialize;
use std::process::Command;

#[derive(Debug, Serialize, Clone)]
pub struct RegistryResult {
    pub success: bool,
    pub message: String,
    pub previous_value: Option<String>,
}

/// Read a DWORD value from registry
pub fn read_dword(hive: &str, path: &str, key: &str) -> Result<u32, String> {
    use winreg::enums::*;
    use winreg::RegKey;

    let hkey = match hive {
        "HKLM" => RegKey::predef(HKEY_LOCAL_MACHINE),
        "HKCU" => RegKey::predef(HKEY_CURRENT_USER),
        "HKCR" => RegKey::predef(HKEY_CLASSES_ROOT),
        _ => return Err(format!("Unknown hive: {}", hive)),
    };

    let subkey = hkey
        .open_subkey(path)
        .map_err(|e| format!("Failed to open key {}: {}", path, e))?;

    let value: u32 = subkey
        .get_value(key)
        .map_err(|e| format!("Failed to read value {}: {}", key, e))?;

    Ok(value)
}

/// Write a DWORD value to registry (creates key if needed)
pub fn write_dword(hive: &str, path: &str, key: &str, value: u32) -> RegistryResult {
    use winreg::enums::*;
    use winreg::RegKey;

    let hkey = match hive {
        "HKLM" => RegKey::predef(HKEY_LOCAL_MACHINE),
        "HKCU" => RegKey::predef(HKEY_CURRENT_USER),
        "HKCR" => RegKey::predef(HKEY_CLASSES_ROOT),
        _ => {
            return RegistryResult {
                success: false,
                message: format!("Unknown hive: {}", hive),
                previous_value: None,
            }
        }
    };

    // Read previous value for undo
    let previous = hkey
        .open_subkey(path)
        .ok()
        .and_then(|k| k.get_value::<u32, _>(key).ok())
        .map(|v| v.to_string());

    // Create or open key with write access
    let (subkey, _) = hkey
        .create_subkey(path)
        .map_err(|e| format!("Failed to create key {}: {}", path, e))
        .unwrap_or_else(|_| {
            // Fallback: try via PowerShell for protected keys
            let _ = write_dword_via_ps(hive, path, key, value);
            return (hkey.open_subkey(path).unwrap(), winreg::enums::RegDisposition::REG_OPENED_EXISTING_KEY);
        });

    match subkey.set_value(key, &value) {
        Ok(_) => RegistryResult {
            success: true,
            message: format!("Set {}\\{}\\{} = {}", hive, path, key, value),
            previous_value: previous,
        },
        Err(e) => {
            // Fallback to PowerShell for protected keys
            let ps_result = write_dword_via_ps(hive, path, key, value);
            if ps_result {
                RegistryResult {
                    success: true,
                    message: format!("Set {}\\{}\\{} = {} (via PowerShell)", hive, path, key, value),
                    previous_value: previous,
                }
            } else {
                RegistryResult {
                    success: false,
                    message: format!("Failed to set value: {}", e),
                    previous_value: previous,
                }
            }
        }
    }
}

/// Read a string value from registry
pub fn read_string(hive: &str, path: &str, key: &str) -> Result<String, String> {
    use winreg::enums::*;
    use winreg::RegKey;

    let hkey = match hive {
        "HKLM" => RegKey::predef(HKEY_LOCAL_MACHINE),
        "HKCU" => RegKey::predef(HKEY_CURRENT_USER),
        "HKCR" => RegKey::predef(HKEY_CLASSES_ROOT),
        _ => return Err(format!("Unknown hive: {}", hive)),
    };

    let subkey = hkey
        .open_subkey(path)
        .map_err(|e| format!("Failed to open key {}: {}", path, e))?;

    let value: String = subkey
        .get_value(key)
        .map_err(|e| format!("Failed to read value {}: {}", key, e))?;

    Ok(value)
}

/// Check if a registry key/value exists
pub fn value_exists(hive: &str, path: &str, key: &str) -> bool {
    read_dword(hive, path, key).is_ok() || read_string(hive, path, key).is_ok()
}

/// Fallback: write via PowerShell for protected registry keys
fn write_dword_via_ps(hive: &str, path: &str, key: &str, value: u32) -> bool {
    let ps_hive = match hive {
        "HKLM" => "HKLM:",
        "HKCU" => "HKCU:",
        "HKCR" => "HKCR:",
        _ => return false,
    };

    let script = format!(
        r#"
        $path = '{}\\{}'
        if (!(Test-Path $path)) {{ New-Item -Path $path -Force | Out-Null }}
        Set-ItemProperty -Path $path -Name '{}' -Value {} -Type DWord -Force
        "#,
        ps_hive, path, key, value
    );

    let output = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-WindowStyle", "Hidden", "-Command", &script])
        .output();

    matches!(output, Ok(o) if o.status.success())
}
