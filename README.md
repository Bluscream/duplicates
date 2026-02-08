# Duplicates Rust 🦀

A high-performance, multi-threaded duplicate file finder and deduplicator written in Rust. Designed for speed, reliability, and ease of use across Windows, Linux, and NAS environments (like Unraid).

## 🚀 Features

- **Blazing Fast**: Uses `rayon` for true multi-threaded file hashing.
- **Progress Tracking**: Real-time progress bars and status updates.
- **Space Aware**: Reports free space before and after deduplication.
- **Smart Cache**: Maintains a compatibility-layer CSV cache (`duplicates.hashes.csv`) to resume scans and avoid re-hashing unchanged files.
- **Multiple Actions**: Supports Symlinking, Hardlinking, and Deleting duplicates.
- **Cross-Platform**: Binaries available for Windows and Linux (Standard and ARM).
- **NAS Optimized**: Tested on Unraid OS and various NAS systems.

## 🛠️ Installation

Download the latest binary for your platform from the [Releases](https://github.com/Bluscream/duplicates-rust/releases) page.

### Linux / NAS
```bash
chmod +x duplicates
./duplicates -p "/path/to/data" -k highest -m symlink
```

### Windows
```powershell
.\duplicates.exe -p "D:\Data" -k highest -m symlink
```

## 📖 Usage

```bash
Usage: duplicates [OPTIONS] --keep <KEEP>

Options:
  -p, --path <PATH>           Search directory (default: .) [default: .]
  -r, --recursive             Recursive search
  -d, --dry-run               Simulation mode
  -k, --keep <KEEP>           Criteria: latest, oldest, highest, deepest, first, last [possible values: latest, oldest, highest, deepest, first, last]
  -m, --mode <MODE>           Action: delete, symlink, hardlink [default: symlink] [possible values: delete, symlink, hardlink]
  -a, --algorithm <ALGORITHM> Algorithm: md5, sha256, sha512, crc32, size, name [default: md5] [possible values: md5, sha256, sha512, crc32, size, name]
  -i, --ignore <IGNORE>       Comma-separated ignore list [default: symlink,.lnk,.url]
  -t, --threads <THREADS>     Parallel hashing threads
  -h, --help                  Print help
  -V, --version               Print version
```

## 🏗️ Building from Source

Requires Rust 1.70+.

```bash
git clone https://github.com/Bluscream/duplicates-rust.git
cd duplicates-rust
cargo build --release
```

## 🤝 Compatibility
The hash cache format is fully compatible with the [PowerShell](duplicates.ps1) and [Bash](duplicates.sh) versions of this tool, allowing you to mix and match tools across different environments while reusing the same scan results.

## 📄 License
MIT
