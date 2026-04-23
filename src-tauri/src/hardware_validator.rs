use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct HardwareInfo {
    pub is_hybrid_cpu: bool,
    pub cpu_name: String,
    pub p_cores: u32,
    pub e_cores: u32,
    pub total_cores: u32,
    pub total_threads: u32,
    pub is_ssd: bool,
    pub has_gpu: bool,
    pub gpu_name: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ValidationResult {
    pub allowed: bool,
    pub warnings: Vec<String>,
    pub blockers: Vec<String>,
}

/// Detect hybrid CPU architecture (Intel 12th gen+)
pub fn detect_hardware() -> HardwareInfo {
    use sysinfo::System;

    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_name = sys
        .cpus()
        .first()
        .map(|c| c.brand().to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let total_threads = sys.cpus().len() as u32;
    let total_cores = sysinfo::System::physical_core_count().unwrap_or(0) as u32;

    // Detect hybrid architecture via PowerShell + WMI
    let (is_hybrid, p_cores, e_cores) = detect_hybrid_topology(total_cores);

    // Check if system drive is SSD
    let is_ssd = check_is_ssd();

    // GPU info
    let gpu_name = crate::system_info::get_system_overview().gpu_name.clone();
    let has_gpu = gpu_name != "Unknown GPU" && !gpu_name.contains("Microsoft Basic");

    HardwareInfo {
        is_hybrid_cpu: is_hybrid,
        cpu_name,
        p_cores,
        e_cores,
        total_cores,
        total_threads,
        is_ssd,
        has_gpu,
        gpu_name,
    }
}

/// Detect P-Core / E-Core topology
fn detect_hybrid_topology(total_cores: u32) -> (bool, u32, u32) {
    // Use PowerShell to query EfficiencyClass from Win32_Processor
    let script = r#"
        try {
            $cores = Get-CimInstance -ClassName Win32_Processor | Select-Object -ExpandProperty NumberOfCores
            $cpuName = (Get-CimInstance -ClassName Win32_Processor).Name
            
            # Check if it's a known hybrid Intel CPU (12th gen+)
            $isHybrid = $cpuName -match 'i[3579]-1[2-9]|i[3579]-[2-9][0-9]|Ultra [579]|Core [579]'
            
            if ($isHybrid) {
                # Estimate P/E core split based on CPU model
                # This is a heuristic - actual detection requires CPUID
                Write-Output "HYBRID:$cores"
            } else {
                Write-Output "STANDARD:$cores"
            }
        } catch {
            Write-Output "ERROR:0"
        }
    "#;

    let result = crate::powershell::execute(script);
    if result.success {
        let output = result.stdout.trim();
        if output.starts_with("HYBRID:") {
            let cores = output.replace("HYBRID:", "").parse::<u32>().unwrap_or(total_cores);
            // Rough estimation: P-cores are typically half or less of total
            let p_cores = (cores * 2) / 3;
            let e_cores = cores - p_cores;
            (true, p_cores.max(1), e_cores.max(1))
        } else {
            (false, total_cores, 0)
        }
    } else {
        (false, total_cores, 0)
    }
}

fn check_is_ssd() -> bool {
    let script = r#"(Get-PhysicalDisk | Where-Object DeviceId -eq 0).MediaType"#;
    let result = crate::powershell::execute(script);
    result.success && result.stdout.to_lowercase().contains("ssd")
}

/// Validate whether a specific tweak is safe for this hardware
pub fn validate_for_tweak(tweak_id: &str, hardware: &HardwareInfo) -> ValidationResult {
    let mut warnings = Vec::new();
    let mut blockers = Vec::new();

    match tweak_id {
        "disable_vbs" | "disable_hvci" => {
            warnings.push("Disabling VBS/HVCI reduces kernel-level security protection.".to_string());
            warnings.push("Not recommended for machines handling sensitive data.".to_string());
        }
        "disable_spectre_meltdown" => {
            warnings.push("Disabling CPU vulnerability mitigations exposes system to potential exploits.".to_string());
            blockers.push("Only recommended for isolated gaming machines.".to_string());
        }
        "gpu_scheduling" => {
            if !hardware.has_gpu {
                blockers.push("No dedicated GPU detected. Cannot enable GPU scheduling.".to_string());
            }
        }
        "pin_to_pcores" => {
            if !hardware.is_hybrid_cpu {
                blockers.push("This CPU does not have a hybrid P-Core/E-Core architecture.".to_string());
            }
        }
        "optimize_ssd" => {
            if !hardware.is_ssd {
                warnings.push("System drive does not appear to be an SSD. Some optimizations may not apply.".to_string());
            }
        }
        _ => {}
    }

    ValidationResult {
        allowed: blockers.is_empty(),
        warnings,
        blockers,
    }
}
