<div align="center">
  <h1>Stomata</h1>

  <pre>
███████╗████████╗ ██████╗ ███╗   ███╗ █████╗ ████████╗ █████╗ 
██╔════╝╚══██╔══╝██╔═══██╗████╗ ████║██╔══██╗╚══██╔══╝██╔══██╗
███████╗   ██║   ██║   ██║██╔████╔██║███████║   ██║   ███████║
╚════██║   ██║   ██║   ██║██║╚██╔╝██║██╔══██║   ██║   ██╔══██║
███████║   ██║   ╚██████╔╝██║ ╚═╝ ██║██║  ██║   ██║   ██║  ██║
╚══════╝   ╚═╝    ╚═════╝ ╚═╝     ╚═╝╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝
  </pre>


  <p>
    <strong>A lightweight real-time terminal system monitor built with Rust</strong>
  </p>
  <p>Track machine resource utilization, processes and performance in real-time as you run from your terminal</p>

  <a href="https://crates.io/crates/stomata-cli">
    <img src="https://img.shields.io/crates/v/stomata-cli.svg" alt="Crates.io">
  </a>
  <a href="https://github.com/aditya172926/stomata-cli/stargazers">
    <img src="https://img.shields.io/github/stars/aditya172926/stomata-cli" alt="GitHub stars">
  </a>
  <a href="https://github.com/aditya172926/stomata-cli/blob/master/LICENSE-MIT">
    <img src="https://img.shields.io/badge/license-MIT%2FApache--2.0-blue" alt="License">
  </a>
</div>

---
## Demo

![stomata demo](https://github.com/user-attachments/assets/eaf887b4-9d51-4d04-9bcc-9693031bde17)

*Stomata tracking memory, swap, CPU, and per-process resource usage in real-time*

---

## Why Stomata?

I repeateadly faced out-of-memory issues while working with very large Rust projects. Other tools showed me the metrics, but not the culprit and in a very clustered UI.

Stomata gives:
- **System-wide metrics**: Memory, Swap, CPU, Disk usage in one place. You can immediately know when the load kicks in
- **Process level details**: See exactly which process is using how much of machine resources
- **Single Process inspection**: Drill into a single process to check its CPU utilizations, memory use, disk read/write bytes in real-time
- **Lightweight**: Minimal footprints

## Features

### System Monitoring
- Real-time memory, swap, CPU, disk usage gauges
- CPU utilization tracking
- OS and system information

### Process Monitoring
- Live process list with resource consumption
- Per-process CPU and memory usage
- Per process meta info about running time, start time, working directory and more

### Process Inspection
- Select any process for detailed view
- Current working directory (CWD)
- Disk read/write bytes with sparkline graphs
- Memory and CPU usage over time

- Designed as a **workspace**: includes a reusable library (`Stomata-core`) and a CLI (`Stomata-cli`)  

---

## Installation

**Via crates.io:**
```bash
cargo install stomata-cli
```

**Using the core library in your project:**
```bash
cargo add stomata-core
```

## Usage

```bash
# Run with default settings
stomata

# Custom refresh interval (milliseconds)
stomata --interval 1000
```

## Stomata Modes
Stomata now comes in 2 modes of operations Interactive and Non-Interactive. Both of these modes implement different features that users can use.

### Non-interactive mode
In this mode, which is the default mode users can use stomata features that don't require a TUI and just want a quick output from the feature.
Currently a non-interactive feature for EVM address validation check is implemented in [Stomata Web3 crate. Example use in README](./stomata-web3/README.md)

### Interactive
In this mode, Stomata cli renders a terminal UI enabling users to see and interact with it. Currently the stomata-core crate implements such features that are interactive.
You can use this command to enable stomata in interactive mode and checkout features available
```
stomata -i
```

## Building from Source

Requires Rust 1.90.0+

```bash
git clone https://github.com/aditya172926/stomata-cli.git
cd stomata-cli

# Debug build
cargo build
# or
make build

# Release build
make release
```

## Project Structure

Stomata is organized as a Cargo workspace:
- `stomata-cli` — The terminal application
- `stomata-core` — Reusable library for metrics collection

## Contributing

Contributions are welcome! Please read our [Contributing Guidelines](CONTRIBUTING.md) before submitting a pull request.

## License

Licensed under

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE))
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT))
