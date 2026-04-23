import { invoke } from '@tauri-apps/api/core';

// === Types ===
export interface SystemOverview {
  os_name: string;
  os_version: string;
  hostname: string;
  cpu_name: string;
  cpu_cores_physical: number;
  cpu_cores_logical: number;
  ram_total_gb: number;
  ram_used_gb: number;
  ram_usage_percent: number;
  gpu_name: string;
  disk_total_gb: number;
  disk_free_gb: number;
  disk_usage_percent: number;
}

export interface LiveMetrics {
  cpu_usage: number;
  ram_usage: number;
  ram_used_gb: number;
  ram_total_gb: number;
}

export interface HardwareInfo {
  is_hybrid_cpu: boolean;
  cpu_name: string;
  p_cores: number;
  e_cores: number;
  total_cores: number;
  total_threads: number;
  is_ssd: boolean;
  has_gpu: boolean;
  gpu_name: string;
}

export interface ValidationResult {
  allowed: boolean;
  warnings: string[];
  blockers: string[];
}

export interface OptimizationItem {
  id: string;
  name: string;
  name_th: string;
  description: string;
  description_th: string;
  category: string;
  risk_level: string;
  is_enabled: boolean;
  requires_reboot: boolean;
  pros: string[];
  cons: string[];
}

export interface OptimizationResult {
  id: string;
  success: boolean;
  message: string;
}

export interface RestorePointResult {
  success: boolean;
  message: string;
}

export interface ActionRecord {
  id: string;
  timestamp: string;
  action_id: string;
  action_name: string;
  category: string;
  previous_value: string | null;
  new_value: string | null;
  registry_path: string | null;
  status: 'Applied' | 'Failed' | 'Reverted';
  reverted: boolean;
}

export interface ProcessInfo {
  pid: number;
  name: string;
  cpu_percent: number;
  memory_mb: number;
  affinity_mask: number;
}

export interface CpuTopology {
  total_cores: number;
  total_threads: number;
  is_hybrid: boolean;
  p_core_mask: number;
  e_core_mask: number;
  cores: CoreInfo[];
}

export interface CoreInfo {
  index: number;
  core_type: string;
  logical_processors: number[];
}

export interface BloatwareApp {
  package_name: string;
  display_name: string;
  publisher: string;
  category: string;
  installed: boolean;
}

// === API Functions ===

export const api = {
  // System
  getSystemOverview: () => invoke<SystemOverview>('get_system_overview'),
  getLiveMetrics: () => invoke<LiveMetrics>('get_live_metrics'),
  getHardwareInfo: () => invoke<HardwareInfo>('get_hardware_info'),
  validateTweak: (tweakId: string) => invoke<ValidationResult>('validate_tweak', { tweakId }),
  getPowerPlan: () => invoke<string>('get_power_plan'),

  // Optimization
  getOptimizationCatalog: () => invoke<OptimizationItem[]>('get_optimization_catalog'),
  getSafeOptimizations: () => invoke<string[]>('get_safe_optimizations'),
  applyOptimization: (id: string) => invoke<OptimizationResult>('apply_optimization', { id }),
  revertOptimization: (id: string) => invoke<OptimizationResult>('revert_optimization', { id }),
  detectOptimizationState: (id: string) => invoke<boolean>('detect_optimization_state', { id }),

  // Restore Point
  createRestorePoint: (description: string) => invoke<RestorePointResult>('create_restore_point', { description }),
  getLatestRestorePoint: () => invoke<string | null>('get_latest_restore_point'),

  // History
  getActionHistory: () => invoke<ActionRecord[]>('get_action_history'),
  clearActionHistory: () => invoke<void>('clear_action_history'),

  // Process Manager
  listProcesses: () => invoke<ProcessInfo[]>('list_processes'),
  getCpuTopology: () => invoke<CpuTopology>('get_cpu_topology'),
  setProcessAffinity: (pid: number, mask: number) => invoke<string>('set_process_affinity', { pid, mask }),
  pinToPCores: (pid: number) => invoke<string>('pin_to_p_cores', { pid }),
  pinToECores: (pid: number) => invoke<string>('pin_to_e_cores', { pid }),
  resetAffinity: (pid: number) => invoke<string>('reset_affinity', { pid }),

  // Bloatware
  scanBloatware: () => invoke<BloatwareApp[]>('scan_bloatware'),
  removeBloatware: (packages: string[]) => invoke<[string, boolean][]>('remove_bloatware', { packages }),
};
