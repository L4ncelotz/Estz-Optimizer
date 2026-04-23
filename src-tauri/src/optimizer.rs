use serde::{Deserialize, Serialize};
use crate::action_store::{ActionStatus, ActionStore};
use crate::registry;
use crate::powershell;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OptimizationItem {
    pub id: String,
    pub name: String,
    pub name_th: String,
    pub description: String,
    pub description_th: String,
    pub category: String,
    pub risk_level: String,
    pub is_enabled: bool,
    pub requires_reboot: bool,
    pub pros: Vec<String>,
    pub cons: Vec<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct OptimizationResult {
    pub id: String,
    pub success: bool,
    pub message: String,
}

/// Detect the current state of an optimization
pub fn detect_state(id: &str) -> bool {
    match id {
        // Privacy
        "disable_telemetry" => {
            registry::read_dword("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DataCollection", "AllowTelemetry")
                .map(|v| v == 0).unwrap_or(false)
        }
        "disable_activity_history" => {
            registry::read_dword("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "EnableActivityFeed")
                .map(|v| v == 0).unwrap_or(false)
        }
        "disable_advertising_id" => {
            registry::read_dword("HKCU", r"Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo", "Enabled")
                .map(|v| v == 0).unwrap_or(false)
        }
        "disable_feedback" => {
            registry::read_dword("HKCU", r"Software\Microsoft\Siuf\Rules", "NumberOfSIUFInPeriod")
                .map(|v| v == 0).unwrap_or(false)
        }
        "disable_location_tracking" => {
            registry::read_dword("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors", "DisableLocation")
                .map(|v| v == 1).unwrap_or(false)
        }
        "disable_tailored_experiences" => {
            registry::read_dword("HKCU", r"Software\Policies\Microsoft\Windows\CloudContent", "DisableTailoredExperiencesWithDiagnosticData")
                .map(|v| v == 1).unwrap_or(false)
        }
        "disable_clipboard_history" => {
            registry::read_dword("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "AllowClipboardHistory")
                .map(|v| v == 0).unwrap_or(false)
        }

        // Performance
        "disable_vbs" => {
            registry::read_dword("HKLM", r"SYSTEM\CurrentControlSet\Control\DeviceGuard", "EnableVirtualizationBasedSecurity")
                .map(|v| v == 0).unwrap_or(false)
        }
        "disable_hvci" => {
            registry::read_dword("HKLM", r"SYSTEM\CurrentControlSet\Control\DeviceGuard\Scenarios\HypervisorEnforcedCodeIntegrity", "Enabled")
                .map(|v| v == 0).unwrap_or(false)
        }
        "disable_power_throttling" => {
            registry::read_dword("HKLM", r"SYSTEM\CurrentControlSet\Control\Power\PowerThrottling", "PowerThrottlingOff")
                .map(|v| v == 1).unwrap_or(false)
        }
        "disable_core_parking" => {
            registry::read_dword("HKLM", r"SYSTEM\CurrentControlSet\Control\Power\PowerSettings\54533251-82be-4824-96c1-47b60b740d00\0cc5b647-c1df-4637-891a-dec35c318583", "ValueMax")
                .map(|v| v == 0).unwrap_or(false)
        }

        // Gaming
        "enable_game_mode" => {
            registry::read_dword("HKCU", r"Software\Microsoft\GameBar", "AllowAutoGameMode")
                .map(|v| v == 1).unwrap_or(true) // Default is enabled
        }
        "disable_game_bar" => {
            registry::read_dword("HKCU", r"Software\Microsoft\Windows\CurrentVersion\GameDVR", "AppCaptureEnabled")
                .map(|v| v == 0).unwrap_or(false)
        }
        "disable_game_dvr" => {
            registry::read_dword("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\GameDVR", "AllowGameDVR")
                .map(|v| v == 0).unwrap_or(false)
        }
        "disable_fullscreen_opt" => {
            registry::read_dword("HKCU", r"System\GameConfigStore", "GameDVR_FSEBehavior")
                .map(|v| v == 2).unwrap_or(false)
        }
        "gpu_scheduling" => {
            registry::read_dword("HKLM", r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers", "HwSchMode")
                .map(|v| v == 2).unwrap_or(false)
        }
        "disable_mouse_accel" => {
            registry::read_dword("HKCU", r"Control Panel\Mouse", "MouseSpeed")
                .map(|v| v == 0).unwrap_or(false)
        }

        // Visual/UI
        "disable_transparency" => {
            registry::read_dword("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize", "EnableTransparency")
                .map(|v| v == 0).unwrap_or(false)
        }
        "disable_animations" => {
            registry::read_dword("HKCU", r"Control Panel\Desktop\WindowMetrics", "MinAnimate")
                .map(|v| v == 0).unwrap_or(false)
        }
        "disable_ads_startmenu" => {
            registry::read_dword("HKCU", r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "ContentDeliveryAllowed")
                .map(|v| v == 0).unwrap_or(false)
        }
        "disable_tips" => {
            registry::read_dword("HKCU", r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SoftLandingEnabled")
                .map(|v| v == 0).unwrap_or(false)
        }
        "disable_news_feed" => {
            registry::read_dword("HKLM", r"SOFTWARE\Policies\Microsoft\Dsh", "AllowNewsAndInterests")
                .map(|v| v == 0).unwrap_or(false)
        }
        "restore_classic_context" => {
            registry::value_exists("HKCU", r"Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\InprocServer32", "")
        }

        // Network
        "disable_nagle" => {
            // Check if TcpNoDelay is set on any interface
            let result = powershell::execute(
                r#"(Get-ItemProperty -Path 'HKLM:\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces\*' -Name 'TcpNoDelay' -ErrorAction SilentlyContinue).TcpNoDelay -contains 1"#
            );
            result.stdout.to_lowercase().contains("true")
        }
        "disable_network_throttling" => {
            registry::read_dword("HKLM", r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile", "NetworkThrottlingIndex")
                .map(|v| v == 0xFFFFFFFF).unwrap_or(false)
        }

        // Boot
        "disable_fast_startup" => {
            registry::read_dword("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Power", "HiberbootEnabled")
                .map(|v| v == 0).unwrap_or(false)
        }

        // SSD
        "optimize_ssd" => {
            !powershell::is_service_running("SysMain")
        }

        _ => false,
    }
}

/// Apply an optimization
pub fn apply(id: &str, store: &mut ActionStore) -> OptimizationResult {
    match id {
        // === PRIVACY ===
        "disable_telemetry" => {
            let r1 = registry::write_dword("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DataCollection", "AllowTelemetry", 0);
            let _ = powershell::execute("Set-Service -Name DiagTrack -StartupType Disabled -ErrorAction SilentlyContinue; Stop-Service -Name DiagTrack -Force -ErrorAction SilentlyContinue");
            store.log_action(id, "Disable Telemetry", "privacy", r1.previous_value.clone(), Some("0".to_string()), Some(r"HKLM\SOFTWARE\Policies\Microsoft\Windows\DataCollection\AllowTelemetry".to_string()), if r1.success { ActionStatus::Applied } else { ActionStatus::Failed });
            OptimizationResult { id: id.to_string(), success: r1.success, message: r1.message }
        }
        "disable_activity_history" => {
            let r1 = registry::write_dword("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "EnableActivityFeed", 0);
            let r2 = registry::write_dword("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "PublishUserActivities", 0);
            store.log_action(id, "Disable Activity History", "privacy", r1.previous_value.clone(), Some("0".to_string()), None, if r1.success && r2.success { ActionStatus::Applied } else { ActionStatus::Failed });
            OptimizationResult { id: id.to_string(), success: r1.success && r2.success, message: r1.message }
        }
        "disable_advertising_id" => {
            let r1 = registry::write_dword("HKCU", r"Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo", "Enabled", 0);
            store.log_action(id, "Disable Advertising ID", "privacy", r1.previous_value.clone(), Some("0".to_string()), None, if r1.success { ActionStatus::Applied } else { ActionStatus::Failed });
            OptimizationResult { id: id.to_string(), success: r1.success, message: r1.message }
        }
        "disable_feedback" => {
            let r1 = registry::write_dword("HKCU", r"Software\Microsoft\Siuf\Rules", "NumberOfSIUFInPeriod", 0);
            store.log_action(id, "Disable Feedback Requests", "privacy", r1.previous_value.clone(), Some("0".to_string()), None, if r1.success { ActionStatus::Applied } else { ActionStatus::Failed });
            OptimizationResult { id: id.to_string(), success: r1.success, message: r1.message }
        }
        "disable_location_tracking" => {
            let r1 = registry::write_dword("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors", "DisableLocation", 1);
            store.log_action(id, "Disable Location Tracking", "privacy", r1.previous_value.clone(), Some("1".to_string()), None, if r1.success { ActionStatus::Applied } else { ActionStatus::Failed });
            OptimizationResult { id: id.to_string(), success: r1.success, message: r1.message }
        }
        "disable_tailored_experiences" => {
            let r1 = registry::write_dword("HKCU", r"Software\Policies\Microsoft\Windows\CloudContent", "DisableTailoredExperiencesWithDiagnosticData", 1);
            store.log_action(id, "Disable Tailored Experiences", "privacy", r1.previous_value.clone(), Some("1".to_string()), None, if r1.success { ActionStatus::Applied } else { ActionStatus::Failed });
            OptimizationResult { id: id.to_string(), success: r1.success, message: r1.message }
        }
        "disable_clipboard_history" => {
            let r1 = registry::write_dword("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "AllowClipboardHistory", 0);
            store.log_action(id, "Disable Cloud Clipboard", "privacy", r1.previous_value.clone(), Some("0".to_string()), None, if r1.success { ActionStatus::Applied } else { ActionStatus::Failed });
            OptimizationResult { id: id.to_string(), success: r1.success, message: r1.message }
        }

        // === PERFORMANCE ===
        "disable_vbs" => {
            let r1 = registry::write_dword("HKLM", r"SYSTEM\CurrentControlSet\Control\DeviceGuard", "EnableVirtualizationBasedSecurity", 0);
            store.log_action(id, "Disable VBS", "performance", r1.previous_value.clone(), Some("0".to_string()), None, if r1.success { ActionStatus::Applied } else { ActionStatus::Failed });
            OptimizationResult { id: id.to_string(), success: r1.success, message: format!("{} (Reboot required)", r1.message) }
        }
        "disable_hvci" => {
            let r1 = registry::write_dword("HKLM", r"SYSTEM\CurrentControlSet\Control\DeviceGuard\Scenarios\HypervisorEnforcedCodeIntegrity", "Enabled", 0);
            store.log_action(id, "Disable HVCI", "performance", r1.previous_value.clone(), Some("0".to_string()), None, if r1.success { ActionStatus::Applied } else { ActionStatus::Failed });
            OptimizationResult { id: id.to_string(), success: r1.success, message: format!("{} (Reboot required)", r1.message) }
        }
        "ultimate_power_plan" => {
            let result = powershell::execute("powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61 2>$null; powercfg -setactive e9a42b02-d5df-448d-aa00-03f14749eb61");
            store.log_action(id, "Ultimate Performance Power Plan", "performance", None, Some("activated".to_string()), None, if result.success { ActionStatus::Applied } else { ActionStatus::Failed });
            OptimizationResult { id: id.to_string(), success: result.success, message: if result.success { "Ultimate Performance plan activated".to_string() } else { result.stderr } }
        }
        "disable_power_throttling" => {
            let r1 = registry::write_dword("HKLM", r"SYSTEM\CurrentControlSet\Control\Power\PowerThrottling", "PowerThrottlingOff", 1);
            store.log_action(id, "Disable Power Throttling", "performance", r1.previous_value.clone(), Some("1".to_string()), None, if r1.success { ActionStatus::Applied } else { ActionStatus::Failed });
            OptimizationResult { id: id.to_string(), success: r1.success, message: r1.message }
        }
        "disable_core_parking" => {
            let result = powershell::execute("powercfg -setacvalueindex scheme_current sub_processor CPMINCORES 100; powercfg -setactive scheme_current");
            store.log_action(id, "Disable Core Parking", "performance", None, Some("100%".to_string()), None, if result.success { ActionStatus::Applied } else { ActionStatus::Failed });
            OptimizationResult { id: id.to_string(), success: result.success, message: if result.success { "Core parking disabled (all cores active)".to_string() } else { result.stderr } }
        }
        "optimize_ssd" => {
            let r1 = powershell::execute("Set-Service -Name SysMain -StartupType Disabled -ErrorAction SilentlyContinue; Stop-Service -Name SysMain -Force -ErrorAction SilentlyContinue");
            store.log_action(id, "SSD Optimization (Disable Superfetch)", "performance", Some("Running".to_string()), Some("Disabled".to_string()), None, if r1.success { ActionStatus::Applied } else { ActionStatus::Failed });
            OptimizationResult { id: id.to_string(), success: r1.success, message: if r1.success { "Superfetch/SysMain disabled for SSD".to_string() } else { r1.stderr } }
        }

        // === GAMING ===
        "enable_game_mode" => {
            let r1 = registry::write_dword("HKCU", r"Software\Microsoft\GameBar", "AllowAutoGameMode", 1);
            store.log_action(id, "Enable Game Mode", "gaming", r1.previous_value.clone(), Some("1".to_string()), None, if r1.success { ActionStatus::Applied } else { ActionStatus::Failed });
            OptimizationResult { id: id.to_string(), success: r1.success, message: r1.message }
        }
        "disable_game_bar" => {
            let r1 = registry::write_dword("HKCU", r"Software\Microsoft\Windows\CurrentVersion\GameDVR", "AppCaptureEnabled", 0);
            let r2 = registry::write_dword("HKCU", r"Software\Microsoft\GameBar", "UseNexusForGameBarEnabled", 0);
            store.log_action(id, "Disable Game Bar", "gaming", r1.previous_value.clone(), Some("0".to_string()), None, if r1.success { ActionStatus::Applied } else { ActionStatus::Failed });
            OptimizationResult { id: id.to_string(), success: r1.success && r2.success, message: r1.message }
        }
        "disable_game_dvr" => {
            let r1 = registry::write_dword("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\GameDVR", "AllowGameDVR", 0);
            store.log_action(id, "Disable Game DVR", "gaming", r1.previous_value.clone(), Some("0".to_string()), None, if r1.success { ActionStatus::Applied } else { ActionStatus::Failed });
            OptimizationResult { id: id.to_string(), success: r1.success, message: r1.message }
        }
        "disable_fullscreen_opt" => {
            let r1 = registry::write_dword("HKCU", r"System\GameConfigStore", "GameDVR_FSEBehavior", 2);
            let r2 = registry::write_dword("HKCU", r"System\GameConfigStore", "GameDVR_FSEBehaviorMode", 2);
            store.log_action(id, "Disable Fullscreen Optimizations", "gaming", r1.previous_value.clone(), Some("2".to_string()), None, if r1.success { ActionStatus::Applied } else { ActionStatus::Failed });
            OptimizationResult { id: id.to_string(), success: r1.success && r2.success, message: r1.message }
        }
        "gpu_scheduling" => {
            let r1 = registry::write_dword("HKLM", r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers", "HwSchMode", 2);
            store.log_action(id, "Enable HW GPU Scheduling", "gaming", r1.previous_value.clone(), Some("2".to_string()), None, if r1.success { ActionStatus::Applied } else { ActionStatus::Failed });
            OptimizationResult { id: id.to_string(), success: r1.success, message: format!("{} (Reboot required)", r1.message) }
        }
        "disable_mouse_accel" => {
            let r1 = registry::write_dword("HKCU", r"Control Panel\Mouse", "MouseSpeed", 0);
            store.log_action(id, "Disable Mouse Acceleration", "gaming", r1.previous_value.clone(), Some("0".to_string()), None, if r1.success { ActionStatus::Applied } else { ActionStatus::Failed });
            OptimizationResult { id: id.to_string(), success: r1.success, message: r1.message }
        }

        // === VISUAL/UI ===
        "disable_transparency" => {
            let r1 = registry::write_dword("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize", "EnableTransparency", 0);
            store.log_action(id, "Disable Transparency", "visual", r1.previous_value.clone(), Some("0".to_string()), None, if r1.success { ActionStatus::Applied } else { ActionStatus::Failed });
            OptimizationResult { id: id.to_string(), success: r1.success, message: r1.message }
        }
        "disable_animations" => {
            let r1 = registry::write_dword("HKCU", r"Control Panel\Desktop\WindowMetrics", "MinAnimate", 0);
            let _ = powershell::execute("Set-ItemProperty -Path 'HKCU:\\Control Panel\\Desktop' -Name 'UserPreferencesMask' -Value ([byte[]](0x90,0x12,0x03,0x80,0x10,0x00,0x00,0x00)) -Type Binary");
            store.log_action(id, "Disable Animations", "visual", r1.previous_value.clone(), Some("0".to_string()), None, if r1.success { ActionStatus::Applied } else { ActionStatus::Failed });
            OptimizationResult { id: id.to_string(), success: r1.success, message: r1.message }
        }
        "disable_ads_startmenu" => {
            let r1 = registry::write_dword("HKCU", r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "ContentDeliveryAllowed", 0);
            let _ = registry::write_dword("HKCU", r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContent-338388Enabled", 0);
            let _ = registry::write_dword("HKCU", r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContent-310093Enabled", 0);
            store.log_action(id, "Disable Start Menu Ads", "visual", r1.previous_value.clone(), Some("0".to_string()), None, if r1.success { ActionStatus::Applied } else { ActionStatus::Failed });
            OptimizationResult { id: id.to_string(), success: r1.success, message: r1.message }
        }
        "disable_tips" => {
            let r1 = registry::write_dword("HKCU", r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SoftLandingEnabled", 0);
            store.log_action(id, "Disable Tips & Suggestions", "visual", r1.previous_value.clone(), Some("0".to_string()), None, if r1.success { ActionStatus::Applied } else { ActionStatus::Failed });
            OptimizationResult { id: id.to_string(), success: r1.success, message: r1.message }
        }
        "disable_news_feed" => {
            let r1 = registry::write_dword("HKLM", r"SOFTWARE\Policies\Microsoft\Dsh", "AllowNewsAndInterests", 0);
            store.log_action(id, "Disable News/Widgets", "visual", r1.previous_value.clone(), Some("0".to_string()), None, if r1.success { ActionStatus::Applied } else { ActionStatus::Failed });
            OptimizationResult { id: id.to_string(), success: r1.success, message: r1.message }
        }
        "restore_classic_context" => {
            let result = powershell::execute(
                r#"New-Item -Path 'HKCU:\Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\InprocServer32' -Force -Value '' | Out-Null"#
            );
            store.log_action(id, "Restore Classic Context Menu", "visual", None, Some("enabled".to_string()), None, if result.success { ActionStatus::Applied } else { ActionStatus::Failed });
            OptimizationResult { id: id.to_string(), success: result.success, message: if result.success { "Classic context menu restored (restart Explorer)".to_string() } else { result.stderr } }
        }

        // === NETWORK ===
        "disable_nagle" => {
            let result = powershell::execute(
                r#"Get-ChildItem 'HKLM:\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces' | ForEach-Object { Set-ItemProperty -Path $_.PSPath -Name 'TcpNoDelay' -Value 1 -Type DWord -Force; Set-ItemProperty -Path $_.PSPath -Name 'TcpAckFrequency' -Value 1 -Type DWord -Force }"#
            );
            store.log_action(id, "Disable Nagle's Algorithm", "network", None, Some("1".to_string()), None, if result.success { ActionStatus::Applied } else { ActionStatus::Failed });
            OptimizationResult { id: id.to_string(), success: result.success, message: if result.success { "Nagle's algorithm disabled on all interfaces".to_string() } else { result.stderr } }
        }
        "disable_network_throttling" => {
            let r1 = registry::write_dword("HKLM", r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile", "NetworkThrottlingIndex", 0xFFFFFFFF);
            store.log_action(id, "Disable Network Throttling", "network", r1.previous_value.clone(), Some("0xFFFFFFFF".to_string()), None, if r1.success { ActionStatus::Applied } else { ActionStatus::Failed });
            OptimizationResult { id: id.to_string(), success: r1.success, message: r1.message }
        }
        "dns_cloudflare" => {
            let result = powershell::execute(
                r#"$adapter = Get-NetAdapter | Where-Object Status -eq 'Up' | Select-Object -First 1; Set-DnsClientServerAddress -InterfaceIndex $adapter.InterfaceIndex -ServerAddresses ('1.1.1.1','1.0.0.1')"#
            );
            store.log_action(id, "Set DNS to Cloudflare", "network", None, Some("1.1.1.1, 1.0.0.1".to_string()), None, if result.success { ActionStatus::Applied } else { ActionStatus::Failed });
            OptimizationResult { id: id.to_string(), success: result.success, message: if result.success { "DNS set to Cloudflare (1.1.1.1)".to_string() } else { result.stderr } }
        }
        "dns_google" => {
            let result = powershell::execute(
                r#"$adapter = Get-NetAdapter | Where-Object Status -eq 'Up' | Select-Object -First 1; Set-DnsClientServerAddress -InterfaceIndex $adapter.InterfaceIndex -ServerAddresses ('8.8.8.8','8.8.4.4')"#
            );
            store.log_action(id, "Set DNS to Google", "network", None, Some("8.8.8.8, 8.8.4.4".to_string()), None, if result.success { ActionStatus::Applied } else { ActionStatus::Failed });
            OptimizationResult { id: id.to_string(), success: result.success, message: if result.success { "DNS set to Google (8.8.8.8)".to_string() } else { result.stderr } }
        }
        "flush_dns" => {
            let result = powershell::execute("Clear-DnsClientCache");
            store.log_action(id, "Flush DNS Cache", "network", None, None, None, if result.success { ActionStatus::Applied } else { ActionStatus::Failed });
            OptimizationResult { id: id.to_string(), success: result.success, message: if result.success { "DNS cache flushed".to_string() } else { result.stderr } }
        }

        // === CLEANUP ===
        "cleanup_temp" => {
            let _result = powershell::execute(
                r#"Remove-Item -Path "$env:TEMP\*" -Recurse -Force -ErrorAction SilentlyContinue; Remove-Item -Path "C:\Windows\Temp\*" -Recurse -Force -ErrorAction SilentlyContinue"#
            );
            store.log_action(id, "Clean Temp Files", "cleanup", None, None, None, ActionStatus::Applied);
            OptimizationResult { id: id.to_string(), success: true, message: "Temp files cleaned".to_string() }
        }
        "cleanup_prefetch" => {
            let _result = powershell::execute(
                r#"Remove-Item -Path "C:\Windows\Prefetch\*" -Force -ErrorAction SilentlyContinue"#
            );
            store.log_action(id, "Clean Prefetch Cache", "cleanup", None, None, None, ActionStatus::Applied);
            OptimizationResult { id: id.to_string(), success: true, message: "Prefetch cache cleaned".to_string() }
        }
        "cleanup_recycle_bin" => {
            let _result = powershell::execute("Clear-RecycleBin -Force -ErrorAction SilentlyContinue");
            store.log_action(id, "Empty Recycle Bin", "cleanup", None, None, None, ActionStatus::Applied);
            OptimizationResult { id: id.to_string(), success: true, message: "Recycle bin emptied".to_string() }
        }

        // === BOOT ===
        "disable_fast_startup" => {
            let r1 = registry::write_dword("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Power", "HiberbootEnabled", 0);
            store.log_action(id, "Disable Fast Startup", "boot", r1.previous_value.clone(), Some("0".to_string()), None, if r1.success { ActionStatus::Applied } else { ActionStatus::Failed });
            OptimizationResult { id: id.to_string(), success: r1.success, message: r1.message }
        }
        "run_sfc" => {
            let result = powershell::execute("sfc /scannow");
            store.log_action(id, "System File Checker", "system", None, None, None, ActionStatus::Applied);
            OptimizationResult { id: id.to_string(), success: result.success, message: result.stdout }
        }
        "run_dism" => {
            let result = powershell::execute("DISM /Online /Cleanup-Image /RestoreHealth");
            store.log_action(id, "DISM Health Restore", "system", None, None, None, ActionStatus::Applied);
            OptimizationResult { id: id.to_string(), success: result.success, message: result.stdout }
        }

        _ => OptimizationResult {
            id: id.to_string(),
            success: false,
            message: format!("Unknown optimization: {}", id),
        },
    }
}

/// Revert an optimization to Windows default
pub fn revert(id: &str, store: &mut ActionStore) -> OptimizationResult {
    // Find the most recent action for this optimization
    let action = store
        .actions
        .iter()
        .filter(|a| a.action_id == id && !a.reverted && a.status == ActionStatus::Applied)
        .last()
        .cloned();

    let result = match id {
        "disable_telemetry" => {
            let _ = registry::write_dword("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DataCollection", "AllowTelemetry", 1);
            let _ = powershell::execute("Set-Service -Name DiagTrack -StartupType Automatic; Start-Service -Name DiagTrack");
            OptimizationResult { id: id.to_string(), success: true, message: "Telemetry re-enabled".to_string() }
        }
        "disable_vbs" => {
            let _ = registry::write_dword("HKLM", r"SYSTEM\CurrentControlSet\Control\DeviceGuard", "EnableVirtualizationBasedSecurity", 1);
            OptimizationResult { id: id.to_string(), success: true, message: "VBS re-enabled (reboot required)".to_string() }
        }
        "disable_hvci" => {
            let _ = registry::write_dword("HKLM", r"SYSTEM\CurrentControlSet\Control\DeviceGuard\Scenarios\HypervisorEnforcedCodeIntegrity", "Enabled", 1);
            OptimizationResult { id: id.to_string(), success: true, message: "HVCI re-enabled (reboot required)".to_string() }
        }
        "disable_game_bar" => {
            let _ = registry::write_dword("HKCU", r"Software\Microsoft\Windows\CurrentVersion\GameDVR", "AppCaptureEnabled", 1);
            let _ = registry::write_dword("HKCU", r"Software\Microsoft\GameBar", "UseNexusForGameBarEnabled", 1);
            OptimizationResult { id: id.to_string(), success: true, message: "Game Bar re-enabled".to_string() }
        }
        "restore_classic_context" => {
            let result = powershell::execute(
                r#"Remove-Item -Path 'HKCU:\Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}' -Recurse -Force -ErrorAction SilentlyContinue"#
            );
            OptimizationResult { id: id.to_string(), success: result.success, message: "Modern context menu restored".to_string() }
        }
        "optimize_ssd" => {
            let _ = powershell::execute("Set-Service -Name SysMain -StartupType Automatic; Start-Service -Name SysMain");
            OptimizationResult { id: id.to_string(), success: true, message: "Superfetch re-enabled".to_string() }
        }
        _ => {
            // Generic revert using stored previous value
            if let Some(act) = &action {
                if let (Some(prev), Some(reg_path)) = (&act.previous_value, &act.registry_path) {
                    if let Ok(prev_val) = prev.parse::<u32>() {
                        let parts: Vec<&str> = reg_path.splitn(3, '\\').collect();
                        if parts.len() == 3 {
                            let _ = registry::write_dword(parts[0], parts[1], parts[2], prev_val);
                        }
                    }
                }
            }
            OptimizationResult { id: id.to_string(), success: true, message: format!("Reverted {}", id) }
        }
    };

    if let Some(act) = action {
        store.mark_reverted(&act.id);
    }

    result
}

/// Get the full catalog of available optimizations with their current states
pub fn get_catalog() -> Vec<OptimizationItem> {
    vec![
        // Privacy
        OptimizationItem { id: "disable_telemetry".into(), name: "Disable Telemetry".into(), name_th: "ปิด Telemetry".into(), description: "Stop Windows from sending diagnostic data to Microsoft".into(), description_th: "หยุด Windows ส่งข้อมูลวินิจฉัยไปยัง Microsoft".into(), category: "privacy".into(), risk_level: "none".into(), is_enabled: detect_state("disable_telemetry"), requires_reboot: false, pros: vec!["Improved privacy".into(), "Less background network usage".into()], cons: vec!["Microsoft can't diagnose issues remotely".into()] },
        OptimizationItem { id: "disable_activity_history".into(), name: "Disable Activity History".into(), name_th: "ปิดประวัติกิจกรรม".into(), description: "Prevent Windows from tracking your app usage and browsing".into(), description_th: "ป้องกัน Windows ติดตามการใช้แอปและการท่องเว็บ".into(), category: "privacy".into(), risk_level: "none".into(), is_enabled: detect_state("disable_activity_history"), requires_reboot: false, pros: vec!["Better privacy".into()], cons: vec!["Timeline feature won't work".into()] },
        OptimizationItem { id: "disable_advertising_id".into(), name: "Disable Advertising ID".into(), name_th: "ปิด Advertising ID".into(), description: "Prevent apps from using your advertising identifier".into(), description_th: "ป้องกันแอปใช้ ID โฆษณาของคุณ".into(), category: "privacy".into(), risk_level: "none".into(), is_enabled: detect_state("disable_advertising_id"), requires_reboot: false, pros: vec!["Less targeted ads".into()], cons: vec!["Ads may be less relevant".into()] },
        OptimizationItem { id: "disable_feedback".into(), name: "Disable Feedback Requests".into(), name_th: "ปิดคำขอ Feedback".into(), description: "Stop Windows from asking for feedback".into(), description_th: "หยุด Windows ถามความคิดเห็น".into(), category: "privacy".into(), risk_level: "none".into(), is_enabled: detect_state("disable_feedback"), requires_reboot: false, pros: vec!["No more popups".into()], cons: vec![] },
        OptimizationItem { id: "disable_location_tracking".into(), name: "Disable Location Tracking".into(), name_th: "ปิดการติดตามตำแหน่ง".into(), description: "Prevent Windows from accessing your location".into(), description_th: "ป้องกัน Windows เข้าถึงตำแหน่งของคุณ".into(), category: "privacy".into(), risk_level: "none".into(), is_enabled: detect_state("disable_location_tracking"), requires_reboot: false, pros: vec!["Better privacy".into()], cons: vec!["Location-based apps won't work".into()] },
        OptimizationItem { id: "disable_tailored_experiences".into(), name: "Disable Tailored Experiences".into(), name_th: "ปิดประสบการณ์ที่ปรับแต่ง".into(), description: "Stop Microsoft from personalizing tips and ads".into(), description_th: "หยุด Microsoft ปรับแต่งเคล็ดลับและโฆษณา".into(), category: "privacy".into(), risk_level: "none".into(), is_enabled: detect_state("disable_tailored_experiences"), requires_reboot: false, pros: vec!["Less data collection".into()], cons: vec![] },
        OptimizationItem { id: "disable_clipboard_history".into(), name: "Disable Cloud Clipboard".into(), name_th: "ปิด Cloud Clipboard".into(), description: "Prevent clipboard data from syncing across devices".into(), description_th: "ป้องกันข้อมูลคลิปบอร์ดซิงค์ข้ามอุปกรณ์".into(), category: "privacy".into(), risk_level: "none".into(), is_enabled: detect_state("disable_clipboard_history"), requires_reboot: false, pros: vec!["Better privacy".into()], cons: vec!["Can't sync clipboard across devices".into()] },

        // Performance
        OptimizationItem { id: "disable_vbs".into(), name: "Disable VBS".into(), name_th: "ปิด VBS".into(), description: "Disable Virtualization-Based Security for better CPU performance".into(), description_th: "ปิด VBS เพื่อประสิทธิภาพ CPU ดีขึ้น".into(), category: "performance".into(), risk_level: "high".into(), is_enabled: detect_state("disable_vbs"), requires_reboot: true, pros: vec!["5-15% CPU performance boost".into(), "Lower latency".into()], cons: vec!["Reduces Credential Guard protection".into(), "Less secure against kernel exploits".into()] },
        OptimizationItem { id: "disable_hvci".into(), name: "Disable HVCI".into(), name_th: "ปิด HVCI".into(), description: "Disable Hypervisor-enforced Code Integrity".into(), description_th: "ปิดการบังคับใช้ Code Integrity โดย Hypervisor".into(), category: "performance".into(), risk_level: "high".into(), is_enabled: detect_state("disable_hvci"), requires_reboot: true, pros: vec!["Better gaming performance".into()], cons: vec!["Less kernel protection".into()] },
        OptimizationItem { id: "ultimate_power_plan".into(), name: "Ultimate Performance Plan".into(), name_th: "แผนพลังงานสูงสุด".into(), description: "Activate the hidden Ultimate Performance power plan".into(), description_th: "เปิดใช้แผนพลังงาน Ultimate Performance ที่ซ่อนอยู่".into(), category: "performance".into(), risk_level: "medium".into(), is_enabled: false, requires_reboot: false, pros: vec!["Maximum CPU/GPU performance".into(), "No power saving delays".into()], cons: vec!["Higher power consumption".into(), "More heat generation".into()] },
        OptimizationItem { id: "disable_power_throttling".into(), name: "Disable Power Throttling".into(), name_th: "ปิด Power Throttling".into(), description: "Prevent Windows from throttling background processes".into(), description_th: "ป้องกัน Windows ลดประสิทธิภาพ process เบื้องหลัง".into(), category: "performance".into(), risk_level: "medium".into(), is_enabled: detect_state("disable_power_throttling"), requires_reboot: false, pros: vec!["Consistent performance".into()], cons: vec!["Higher power usage".into()] },
        OptimizationItem { id: "disable_core_parking".into(), name: "Disable Core Parking".into(), name_th: "ปิด Core Parking".into(), description: "Keep all CPU cores active at all times".into(), description_th: "เปิดใช้งาน CPU ทุกคอร์ตลอดเวลา".into(), category: "performance".into(), risk_level: "medium".into(), is_enabled: detect_state("disable_core_parking"), requires_reboot: false, pros: vec!["Faster thread scheduling".into()], cons: vec!["Higher idle power consumption".into()] },
        OptimizationItem { id: "optimize_ssd".into(), name: "SSD Optimization".into(), name_th: "ปรับแต่ง SSD".into(), description: "Disable Superfetch/SysMain (unnecessary for SSDs)".into(), description_th: "ปิด Superfetch/SysMain (ไม่จำเป็นสำหรับ SSD)".into(), category: "performance".into(), risk_level: "low".into(), is_enabled: detect_state("optimize_ssd"), requires_reboot: false, pros: vec!["Less disk writes on SSD".into(), "Lower RAM usage".into()], cons: vec!["Slightly slower app launch on HDD".into()] },

        // Gaming
        OptimizationItem { id: "enable_game_mode".into(), name: "Enable Game Mode".into(), name_th: "เปิด Game Mode".into(), description: "Enable Windows Game Mode for better gaming performance".into(), description_th: "เปิด Game Mode สำหรับประสิทธิภาพเกมที่ดีขึ้น".into(), category: "gaming".into(), risk_level: "none".into(), is_enabled: detect_state("enable_game_mode"), requires_reboot: false, pros: vec!["Prioritized GPU/CPU for games".into()], cons: vec![] },
        OptimizationItem { id: "disable_game_bar".into(), name: "Disable Game Bar".into(), name_th: "ปิด Game Bar".into(), description: "Disable Xbox Game Bar overlay".into(), description_th: "ปิด Xbox Game Bar overlay".into(), category: "gaming".into(), risk_level: "low".into(), is_enabled: detect_state("disable_game_bar"), requires_reboot: false, pros: vec!["Less overhead while gaming".into()], cons: vec!["Can't use Win+G overlay".into()] },
        OptimizationItem { id: "disable_game_dvr".into(), name: "Disable Game DVR".into(), name_th: "ปิด Game DVR".into(), description: "Disable background game recording".into(), description_th: "ปิดการบันทึกเกมเบื้องหลัง".into(), category: "gaming".into(), risk_level: "low".into(), is_enabled: detect_state("disable_game_dvr"), requires_reboot: false, pros: vec!["More GPU/CPU resources for games".into()], cons: vec!["Can't record gameplay clips".into()] },
        OptimizationItem { id: "disable_fullscreen_opt".into(), name: "Disable Fullscreen Optimizations".into(), name_th: "ปิด Fullscreen Optimizations".into(), description: "Disable DWM fullscreen optimizations globally".into(), description_th: "ปิด DWM fullscreen optimizations ทั้งระบบ".into(), category: "gaming".into(), risk_level: "low".into(), is_enabled: detect_state("disable_fullscreen_opt"), requires_reboot: false, pros: vec!["Better FPS in some games".into(), "Reduced input lag".into()], cons: vec!["Alt-tab may be slower".into()] },
        OptimizationItem { id: "gpu_scheduling".into(), name: "HW GPU Scheduling".into(), name_th: "GPU Scheduling แบบ Hardware".into(), description: "Enable hardware-accelerated GPU scheduling".into(), description_th: "เปิด GPU scheduling แบบ hardware-accelerated".into(), category: "gaming".into(), risk_level: "medium".into(), is_enabled: detect_state("gpu_scheduling"), requires_reboot: true, pros: vec!["Lower GPU latency".into(), "Better frame pacing".into()], cons: vec!["May cause issues on older GPUs".into()] },
        OptimizationItem { id: "disable_mouse_accel".into(), name: "Disable Mouse Acceleration".into(), name_th: "ปิด Mouse Acceleration".into(), description: "Disable 'Enhance pointer precision' for raw mouse input".into(), description_th: "ปิด 'Enhance pointer precision' เพื่อใช้ raw mouse input".into(), category: "gaming".into(), risk_level: "none".into(), is_enabled: detect_state("disable_mouse_accel"), requires_reboot: false, pros: vec!["Consistent mouse movement".into(), "Better aim in FPS games".into()], cons: vec!["Mouse may feel different at first".into()] },

        // Visual
        OptimizationItem { id: "disable_transparency".into(), name: "Disable Transparency".into(), name_th: "ปิดความโปร่งใส".into(), description: "Disable transparency effects in Windows".into(), description_th: "ปิด effect ความโปร่งใสใน Windows".into(), category: "visual".into(), risk_level: "none".into(), is_enabled: detect_state("disable_transparency"), requires_reboot: false, pros: vec!["Less GPU usage".into()], cons: vec!["UI looks less modern".into()] },
        OptimizationItem { id: "disable_animations".into(), name: "Disable Animations".into(), name_th: "ปิด Animation".into(), description: "Disable window animations for snappier UI".into(), description_th: "ปิด animation หน้าต่างให้ UI ตอบสนองเร็วขึ้น".into(), category: "visual".into(), risk_level: "none".into(), is_enabled: detect_state("disable_animations"), requires_reboot: false, pros: vec!["Faster UI response".into()], cons: vec!["UI transitions are instant (no smooth effect)".into()] },
        OptimizationItem { id: "disable_ads_startmenu".into(), name: "Disable Start Menu Ads".into(), name_th: "ปิดโฆษณา Start Menu".into(), description: "Remove suggested apps and ads from Start Menu".into(), description_th: "ลบแอปแนะนำและโฆษณาจาก Start Menu".into(), category: "visual".into(), risk_level: "none".into(), is_enabled: detect_state("disable_ads_startmenu"), requires_reboot: false, pros: vec!["Cleaner Start Menu".into()], cons: vec![] },
        OptimizationItem { id: "disable_tips".into(), name: "Disable Tips & Suggestions".into(), name_th: "ปิดเคล็ดลับ".into(), description: "Stop Windows from showing tips and suggestions".into(), description_th: "หยุด Windows แสดงเคล็ดลับ".into(), category: "visual".into(), risk_level: "none".into(), is_enabled: detect_state("disable_tips"), requires_reboot: false, pros: vec!["No more tip popups".into()], cons: vec![] },
        OptimizationItem { id: "disable_news_feed".into(), name: "Disable News/Widgets".into(), name_th: "ปิด News/Widgets".into(), description: "Disable News and Interests / Widgets on taskbar".into(), description_th: "ปิด News and Interests / Widgets บน taskbar".into(), category: "visual".into(), risk_level: "none".into(), is_enabled: detect_state("disable_news_feed"), requires_reboot: false, pros: vec!["Cleaner taskbar".into(), "Less RAM usage".into()], cons: vec!["No quick news access".into()] },
        OptimizationItem { id: "restore_classic_context".into(), name: "Classic Context Menu".into(), name_th: "เมนูคลิกขวาแบบคลาสสิก".into(), description: "Restore the full right-click context menu (Windows 11)".into(), description_th: "คืนเมนูคลิกขวาแบบเต็ม (Windows 11)".into(), category: "visual".into(), risk_level: "low".into(), is_enabled: detect_state("restore_classic_context"), requires_reboot: false, pros: vec!["Full context menu without 'Show more options'".into()], cons: vec!["Looks less modern".into()] },

        // Network
        OptimizationItem { id: "disable_nagle".into(), name: "Disable Nagle's Algorithm".into(), name_th: "ปิด Nagle's Algorithm".into(), description: "Reduce network latency by disabling packet coalescing".into(), description_th: "ลด latency เครือข่ายด้วยการปิดการรวม packet".into(), category: "network".into(), risk_level: "low".into(), is_enabled: detect_state("disable_nagle"), requires_reboot: false, pros: vec!["Lower ping in games".into(), "Faster small packet delivery".into()], cons: vec!["Slightly more network overhead".into()] },
        OptimizationItem { id: "disable_network_throttling".into(), name: "Disable Network Throttling".into(), name_th: "ปิด Network Throttling".into(), description: "Remove Windows network bandwidth throttling for multimedia".into(), description_th: "ลบการจำกัดแบนด์วิดท์เครือข่ายสำหรับมัลติมีเดีย".into(), category: "network".into(), risk_level: "low".into(), is_enabled: detect_state("disable_network_throttling"), requires_reboot: false, pros: vec!["Full network bandwidth available".into()], cons: vec![] },
        OptimizationItem { id: "flush_dns".into(), name: "Flush DNS Cache".into(), name_th: "ล้าง DNS Cache".into(), description: "Clear the DNS resolver cache".into(), description_th: "ล้างแคช DNS resolver".into(), category: "network".into(), risk_level: "none".into(), is_enabled: false, requires_reboot: false, pros: vec!["Resolve DNS issues".into()], cons: vec![] },

        // Boot
        OptimizationItem { id: "disable_fast_startup".into(), name: "Disable Fast Startup".into(), name_th: "ปิด Fast Startup".into(), description: "Disable hybrid shutdown for cleaner boots".into(), description_th: "ปิดการ shutdown แบบ hybrid เพื่อ boot ที่สะอาดขึ้น".into(), category: "boot".into(), risk_level: "low".into(), is_enabled: detect_state("disable_fast_startup"), requires_reboot: false, pros: vec!["Cleaner boot process".into(), "Fixes dual-boot issues".into()], cons: vec!["Slightly slower cold boot".into()] },

        // Cleanup
        OptimizationItem { id: "cleanup_temp".into(), name: "Clean Temp Files".into(), name_th: "ลบไฟล์ Temp".into(), description: "Remove temporary files from Windows and user temp folders".into(), description_th: "ลบไฟล์ชั่วคราวจากโฟลเดอร์ Temp".into(), category: "cleanup".into(), risk_level: "none".into(), is_enabled: false, requires_reboot: false, pros: vec!["Free disk space".into()], cons: vec![] },
        OptimizationItem { id: "cleanup_prefetch".into(), name: "Clean Prefetch".into(), name_th: "ลบ Prefetch".into(), description: "Clear the Windows Prefetch cache".into(), description_th: "ล้างแคช Prefetch ของ Windows".into(), category: "cleanup".into(), risk_level: "none".into(), is_enabled: false, requires_reboot: false, pros: vec!["Free disk space".into()], cons: vec!["First app launch may be slightly slower".into()] },
        OptimizationItem { id: "cleanup_recycle_bin".into(), name: "Empty Recycle Bin".into(), name_th: "ล้างถังขยะ".into(), description: "Empty the Recycle Bin".into(), description_th: "ล้างถังขยะ".into(), category: "cleanup".into(), risk_level: "none".into(), is_enabled: false, requires_reboot: false, pros: vec!["Free disk space".into()], cons: vec!["Permanently deletes recycled files".into()] },

        // System
        OptimizationItem { id: "run_sfc".into(), name: "System File Checker".into(), name_th: "ตรวจสอบไฟล์ระบบ".into(), description: "Scan and repair corrupted Windows system files".into(), description_th: "สแกนและซ่อมไฟล์ระบบ Windows ที่เสียหาย".into(), category: "system".into(), risk_level: "none".into(), is_enabled: false, requires_reboot: false, pros: vec!["Repairs corrupted files".into()], cons: vec!["May take several minutes".into()] },
        OptimizationItem { id: "run_dism".into(), name: "DISM Health Restore".into(), name_th: "DISM ซ่อมแซมระบบ".into(), description: "Repair Windows component store using DISM".into(), description_th: "ซ่อมแซม Windows component store ด้วย DISM".into(), category: "system".into(), risk_level: "none".into(), is_enabled: false, requires_reboot: false, pros: vec!["Fixes Windows Update issues".into()], cons: vec!["May take 10-15 minutes".into()] },
    ]
}

/// Get IDs for safe 1-click optimization
pub fn get_safe_optimization_ids() -> Vec<String> {
    vec![
        "disable_telemetry".into(),
        "disable_activity_history".into(),
        "disable_advertising_id".into(),
        "disable_feedback".into(),
        "disable_location_tracking".into(),
        "disable_tailored_experiences".into(),
        "disable_ads_startmenu".into(),
        "disable_tips".into(),
        "disable_news_feed".into(),
        "cleanup_temp".into(),
        "cleanup_prefetch".into(),
    ]
}
