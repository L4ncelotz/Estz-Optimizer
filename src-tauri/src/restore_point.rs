use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct RestorePointResult {
    pub success: bool,
    pub message: String,
}

/// Create a System Restore Point (handles 24h frequency limit)
pub fn create_restore_point(description: &str) -> RestorePointResult {
    // First ensure System Restore is enabled on C:
    let enable_result = crate::powershell::execute(
        "Enable-ComputerRestore -Drive 'C:\\' -ErrorAction SilentlyContinue"
    );

    if !enable_result.success {
        // Not fatal, it might already be enabled
    }

    // Temporarily disable the 24-hour frequency limit
    let freq_script = r#"
        $regPath = 'HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion\SystemRestore'
        Set-ItemProperty -Path $regPath -Name 'SystemRestorePointCreationFrequency' -Value 0 -Type DWord -Force -ErrorAction SilentlyContinue
    "#;
    let _ = crate::powershell::execute(freq_script);

    // Create the restore point
    let create_script = format!(
        r#"Checkpoint-Computer -Description 'Estz: {}' -RestorePointType 'MODIFY_SETTINGS' -ErrorAction Stop"#,
        description.replace('\'', "''")
    );

    let result = crate::powershell::execute(&create_script);

    if result.success {
        RestorePointResult {
            success: true,
            message: format!("Restore point 'Estz: {}' created successfully", description),
        }
    } else {
        // Fallback: try via WMI
        let wmi_script = format!(
            r#"
            $description = 'Estz: {}'
            $restorePoint = [wmiclass]"\\.\root\default:SystemRestore"
            $result = $restorePoint.CreateRestorePoint($description, 12, 100)
            if ($result.ReturnValue -eq 0) {{ Write-Output 'SUCCESS' }} else {{ Write-Output "FAILED:$($result.ReturnValue)" }}
            "#,
            description.replace('\'', "''")
        );

        let wmi_result = crate::powershell::execute(&wmi_script);
        if wmi_result.stdout.contains("SUCCESS") {
            RestorePointResult {
                success: true,
                message: format!("Restore point 'Estz: {}' created via WMI", description),
            }
        } else {
            RestorePointResult {
                success: false,
                message: format!(
                    "Failed to create restore point: {} | {}",
                    result.stderr, wmi_result.stderr
                ),
            }
        }
    }
}

/// Get the latest restore point info
pub fn get_latest_restore_point() -> Option<String> {
    let script = r#"
        $rp = Get-ComputerRestorePoint | Sort-Object -Property SequenceNumber -Descending | Select-Object -First 1
        if ($rp) {
            Write-Output "$($rp.Description) | $($rp.CreationTime)"
        }
    "#;
    let result = crate::powershell::execute(script);
    if result.success && !result.stdout.is_empty() {
        Some(result.stdout)
    } else {
        None
    }
}
