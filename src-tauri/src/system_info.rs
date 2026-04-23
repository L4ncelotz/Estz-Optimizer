use serde::Serialize;
use std::process::Command;

#[derive(Debug, Serialize, Clone)]
pub struct SystemOverview {
    pub os_name: String,
    pub os_version: String,
    pub hostname: String,
    pub cpu_name: String,
    pub cpu_cores_physical: usize,
    pub cpu_cores_logical: usize,
    pub ram_total_gb: f64,
    pub ram_used_gb: f64,
    pub ram_usage_percent: f64,
    pub gpu_name: String,
    pub disk_total_gb: f64,
    pub disk_free_gb: f64,
    pub disk_usage_percent: f64,
}

#[derive(Debug, Serialize, Clone)]
pub struct LiveMetrics {
    pub cpu_usage: f64,
    pub ram_usage: f64,
    pub ram_used_gb: f64,
    pub ram_total_gb: f64,
}

pub fn get_system_overview() -> SystemOverview {
    use sysinfo::{System, Disks};

    let mut sys = System::new_all();
    sys.refresh_all();

    let os_name = System::name().unwrap_or_else(|| "Windows".to_string());
    let os_version = System::os_version().unwrap_or_else(|| "Unknown".to_string());
    let hostname = System::host_name().unwrap_or_else(|| "Unknown".to_string());

    let cpu_name = sys
        .cpus()
        .first()
        .map(|c| c.brand().to_string())
        .unwrap_or_else(|| "Unknown CPU".to_string());

    let cpu_cores_physical = sysinfo::System::physical_core_count().unwrap_or(0);
    let cpu_cores_logical = sys.cpus().len();

    let ram_total = sys.total_memory();
    let ram_used = sys.used_memory();
    let ram_total_gb = ram_total as f64 / 1_073_741_824.0;
    let ram_used_gb = ram_used as f64 / 1_073_741_824.0;
    let ram_usage_percent = if ram_total > 0 {
        (ram_used as f64 / ram_total as f64) * 100.0
    } else {
        0.0
    };

    let gpu_name = get_gpu_name();

    let disks = Disks::new_with_refreshed_list();
    let system_disk = disks
        .iter()
        .find(|d| {
            d.mount_point()
                .to_str()
                .map(|s| s.starts_with("C:"))
                .unwrap_or(false)
        });

    let (disk_total_gb, disk_free_gb, disk_usage_percent) = match system_disk {
        Some(disk) => {
            let total = disk.total_space() as f64 / 1_073_741_824.0;
            let free = disk.available_space() as f64 / 1_073_741_824.0;
            let usage = if total > 0.0 {
                ((total - free) / total) * 100.0
            } else {
                0.0
            };
            (total, free, usage)
        }
        None => (0.0, 0.0, 0.0),
    };

    SystemOverview {
        os_name,
        os_version,
        hostname,
        cpu_name,
        cpu_cores_physical,
        cpu_cores_logical,
        ram_total_gb: (ram_total_gb * 10.0).round() / 10.0,
        ram_used_gb: (ram_used_gb * 10.0).round() / 10.0,
        ram_usage_percent: (ram_usage_percent * 10.0).round() / 10.0,
        gpu_name,
        disk_total_gb: (disk_total_gb * 10.0).round() / 10.0,
        disk_free_gb: (disk_free_gb * 10.0).round() / 10.0,
        disk_usage_percent: (disk_usage_percent * 10.0).round() / 10.0,
    }
}

pub fn get_live_metrics() -> LiveMetrics {
    use sysinfo::System;

    let mut sys = System::new();
    sys.refresh_cpu_all();
    std::thread::sleep(std::time::Duration::from_millis(200));
    sys.refresh_cpu_all();
    sys.refresh_memory();

    let cpu_usage = sys.global_cpu_usage() as f64;
    let ram_total = sys.total_memory();
    let ram_used = sys.used_memory();
    let ram_total_gb = ram_total as f64 / 1_073_741_824.0;
    let ram_used_gb = ram_used as f64 / 1_073_741_824.0;
    let ram_usage = if ram_total > 0 {
        (ram_used as f64 / ram_total as f64) * 100.0
    } else {
        0.0
    };

    LiveMetrics {
        cpu_usage: (cpu_usage * 10.0).round() / 10.0,
        ram_usage: (ram_usage * 10.0).round() / 10.0,
        ram_used_gb: (ram_used_gb * 10.0).round() / 10.0,
        ram_total_gb: (ram_total_gb * 10.0).round() / 10.0,
    }
}

fn get_gpu_name() -> String {
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            "(Get-CimInstance Win32_VideoController | Select-Object -First 1).Name",
        ])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let name = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if name.is_empty() {
                "Unknown GPU".to_string()
            } else {
                name
            }
        }
        _ => "Unknown GPU".to_string(),
    }
}
