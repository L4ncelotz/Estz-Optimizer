use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_percent: f64,
    pub memory_mb: f64,
    pub affinity_mask: u64,
}

#[derive(Debug, Serialize, Clone)]
pub struct CpuTopology {
    pub total_cores: u32,
    pub total_threads: u32,
    pub is_hybrid: bool,
    pub p_core_mask: u64,
    pub e_core_mask: u64,
    pub cores: Vec<CoreInfo>,
}

#[derive(Debug, Serialize, Clone)]
pub struct CoreInfo {
    pub index: u32,
    pub core_type: String, // "P-Core" or "E-Core" or "Standard"
    pub logical_processors: Vec<u32>,
}

/// Get list of running processes with CPU/memory info
pub fn list_processes() -> Vec<ProcessInfo> {
    use sysinfo::System;

    let mut sys = System::new();
    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
    std::thread::sleep(std::time::Duration::from_millis(200));
    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

    let mut processes: Vec<ProcessInfo> = sys
        .processes()
        .iter()
        .filter(|(_, p)| !p.name().to_string_lossy().is_empty())
        .map(|(pid, p)| {
            ProcessInfo {
                pid: pid.as_u32(),
                name: p.name().to_string_lossy().to_string(),
                cpu_percent: (p.cpu_usage() as f64 * 10.0).round() / 10.0,
                memory_mb: (p.memory() as f64 / 1_048_576.0 * 10.0).round() / 10.0,
                affinity_mask: 0, // Will be populated separately if needed
            }
        })
        .collect();

    processes.sort_by(|a, b| b.cpu_percent.partial_cmp(&a.cpu_percent).unwrap_or(std::cmp::Ordering::Equal));
    processes.truncate(100); // Top 100 processes
    processes
}

/// Get CPU topology (P-Core / E-Core mapping)
pub fn get_cpu_topology() -> CpuTopology {
    let hw = crate::hardware_validator::detect_hardware();
    
    let total_threads = hw.total_threads;
    let total_cores = hw.total_cores;
    
    let mut cores = Vec::new();
    
    if hw.is_hybrid_cpu {
        // P-Cores typically come first in the processor numbering
        let threads_per_p_core = 2u32; // P-cores are hyperthreaded
        let p_core_threads = hw.p_cores * threads_per_p_core;
        
        let mut p_core_mask: u64 = 0;
        let mut e_core_mask: u64 = 0;
        
        for i in 0..total_cores {
            if i < hw.p_cores {
                let lp1 = i * 2;
                let lp2 = i * 2 + 1;
                p_core_mask |= (1u64 << lp1) | (1u64 << lp2);
                cores.push(CoreInfo {
                    index: i,
                    core_type: "P-Core".to_string(),
                    logical_processors: vec![lp1, lp2],
                });
            } else {
                let lp = p_core_threads + (i - hw.p_cores);
                e_core_mask |= 1u64 << lp;
                cores.push(CoreInfo {
                    index: i,
                    core_type: "E-Core".to_string(),
                    logical_processors: vec![lp],
                });
            }
        }
        
        CpuTopology {
            total_cores,
            total_threads,
            is_hybrid: true,
            p_core_mask,
            e_core_mask,
            cores,
        }
    } else {
        let all_mask = if total_threads >= 64 { u64::MAX } else { (1u64 << total_threads) - 1 };
        
        for i in 0..total_cores {
            let threads_per_core = total_threads / total_cores;
            let lps: Vec<u32> = (0..threads_per_core).map(|t| i * threads_per_core + t).collect();
            cores.push(CoreInfo {
                index: i,
                core_type: "Standard".to_string(),
                logical_processors: lps,
            });
        }
        
        CpuTopology {
            total_cores,
            total_threads,
            is_hybrid: false,
            p_core_mask: all_mask,
            e_core_mask: 0,
            cores,
        }
    }
}

/// Set process affinity mask
pub fn set_process_affinity(pid: u32, mask: u64) -> Result<String, String> {
    let script = format!(
        r#"
        try {{
            $process = Get-Process -Id {} -ErrorAction Stop
            $process.ProcessorAffinity = [IntPtr]{}
            Write-Output "SUCCESS"
        }} catch {{
            Write-Output "FAILED: $_"
        }}
        "#,
        pid, mask
    );

    let result = crate::powershell::execute(&script);
    if result.stdout.contains("SUCCESS") {
        Ok(format!("Affinity set for PID {} to mask 0x{:X}", pid, mask))
    } else {
        Err(format!("Failed to set affinity: {}", result.stdout))
    }
}

/// Pin a process to P-Cores only
pub fn pin_to_p_cores(pid: u32) -> Result<String, String> {
    let topology = get_cpu_topology();
    if !topology.is_hybrid {
        return Err("CPU does not have hybrid architecture".to_string());
    }
    set_process_affinity(pid, topology.p_core_mask)
}

/// Pin a process to E-Cores only
pub fn pin_to_e_cores(pid: u32) -> Result<String, String> {
    let topology = get_cpu_topology();
    if !topology.is_hybrid {
        return Err("CPU does not have hybrid architecture".to_string());
    }
    set_process_affinity(pid, topology.e_core_mask)
}

/// Reset process to use all cores
pub fn reset_affinity(pid: u32) -> Result<String, String> {
    let topology = get_cpu_topology();
    let all_mask = if topology.total_threads >= 64 { u64::MAX } else { (1u64 << topology.total_threads) - 1 };
    set_process_affinity(pid, all_mask)
}
