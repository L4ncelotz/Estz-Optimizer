<div align="center">
  
# 🚀 Estz Windows Optimizer
### Extreme Windows Performance Optimizer built with Rust & React

<p align="center">
  <img src="https://img.shields.io/badge/Platform-Windows%2010%20%7C%2011-blue?style=for-the-badge&logo=windows" />
  <img src="https://img.shields.io/badge/Tauri-v2.0-FFC131?style=for-the-badge&logo=tauri&logoColor=white" />
  <img src="https://img.shields.io/badge/Language-Rust%20%7C%20React-1b1c1d?style=for-the-badge" />
</p>

</div>

---

**Estz** is an enterprise-grade Windows optimization tool engineered to maximize operating system performance. Built natively with **Rust (Tauri)** for zero-latency system calls, and **React + TypeScript + TailwindCSS** for a seamless, glassmorphism UI.

## 🔥 Core Features

- ⚡ **Zero-Latency Telemetry**: Real-time CPU, RAM, and Disk polling with zero thread-blocking via Rust's `OnceLock` global memory states.
- 🧠 **Hybrid CPU Topology Pinning**: Advanced Process Manager that automatically detects Intel Big.LITTLE (P-Core/E-Core) topologies. Users can force-pin specific processes to pure Performance Cores for gaming.
- 🛡️ **Safe Optimize (1-Click)**: Fully reversible system tweaks ranging from Registry deep-cleans, telemetry disabling, and SSD optimizations. Creates System Restore points automatically prior to execution.
- 🗑️ **OEM Bloatware Purger**: Detects and purges pre-installed Appx packages and hidden Windows bloatware instantly.

## 🛠️ Technology Stack
- **Interface**: React, TypeScript, Vite, Tailwind CSS, Lucide Icons
- **State Management**: TanStack React Query (Zero-re-render data fetching)
- **Backend**: Rust, Tauri 2.0, sysinfo
- **OS Interop**: Native PowerShell integrations with advanced parameter sanitization.

## 📥 Installation

1. Navigate to the [Releases Tab](https://github.com/L4ncelotz/Estz-Optimizer/releases).
2. Download `Estz_v0.1.X_Setup_x64.exe` for standard installation, or `Estz_v0.1.X_Portable_x64.exe` to run immediately without installing.
3. *Requires Administrator Privileges on launch*.

## 🏗️ Building from Source

Ensure you have [Node.js](https://nodejs.org/), [Rust](https://rustup.rs/), and the [Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) installed.

```bash
git clone https://github.com/L4ncelotz/Estz-Optimizer
cd Estz-Optimizer
npm install

# Run in dev mode
npm run tauri dev

# Compile Production Build
npm run tauri build
```

## 📜 License
Distributed under the MIT License.
