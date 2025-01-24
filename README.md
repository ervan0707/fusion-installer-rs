# VMware Fusion Downloader (fusion-installer-rs)

A Rust-based command-line tool for downloading VMware Fusion without requiring a Business/Commercial account. This tool automatically downloads the latest version (or a specified version) of VMware Fusion, extracts it, and prepares it for installation.

## Features

- 🚀 Download the latest version of VMware Fusion automatically
- 📦 Specify a particular version for download
- 🔍 System compatibility checks for macOS
- 📊 Progress bar for download tracking
- 🧹 Automatic cleanup and preparation
- 🔒 Handles quarantine attributes
- 💡 License file detection and management

## Prerequisites

- Nix package manager with flakes enabled
- macOS 11 or higher (for VMware Fusion 13.0.0+)
- Command line tools: `tar`, `unzip`

## Installation

### Using Nix Flakes (Recommended)

```bash
# Run directly
nix run github:Ervan0707/fusion-installer-rs

# Or install to your profile
nix profile install github:Ervan0707/fusion-installer-rs
```

### Building from Source

Clone the repository and build using Nix:

```bash
git clone https://github.com/Ervan0707/fusion-installer-rs.git
cd fusion-installer-rs
nix build
```

The compiled binary will be available at `./result/bin/fusion-installer-rs`

## Development

Enter development shell:

```bash
nix develop
```

Build the project:

```bash
nix build
```

## Usage

### Basic Usage

Download the latest version:
```bash
fusion-installer-rs
```

### Command Line Options

```bash
# Show help
fusion-installer-rs --help

# Keep the downloaded file compressed (don't extract)
fusion-installer-rs -k

# Specify a particular version
fusion-installer-rs -v 13.0.0
```

### Options

- `-k, --keep-compressed`: Keep the downloaded file compressed and skip extraction
- `-v, --version <VERSION>`: Specify the VMware Fusion version to download (13.0.0 or higher)

## How It Works

1. Checks system compatibility (macOS version)
2. Fetches available versions from VMware's servers
3. Downloads the specified or latest version
4. Extracts and prepares the application
5. Removes quarantine attributes
6. Cleans up temporary files
7. Checks for existing license files
