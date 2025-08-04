<h1 align="center">Wormhole</h1>
<p align="center"><b>A wormhole to transfer files</b></p>
<p align="center">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust Badge">
  <img src="https://img.shields.io/badge/MIT-green?style=for-the-badge" alt="MIT License Badge">
  <img src="https://img.shields.io/badge/Windows%20Terminal-4D4D4D?style=for-the-badge&logo=windows%20terminal&logoColor=white" alt="Windows Terminal Badge">
  <img src="https://img.shields.io/badge/GNU%20Bash-4EAA25?style=for-the-badge&logo=GNU%20Bash&logoColor=white" alt="GNU Bash Badge">
  <img src="https://img.shields.io/badge/iTerm2-000000?style=for-the-badge&logo=iterm2&logoColor=white" alt="iTerm2 Badge">
</p>
<p align="center">
  <img src="https://raw.githubusercontent.com/ByteJoseph/ByteJoseph/refs/heads/main/.github/preview-wormhole.png" alt="Wormhole Preview" >
</p>


Wormhole is a CLI tool that allows users to transfer files to devices on the same local network. It generates a QR code that can be scanned by any phone or browser-enabled device, enabling simple and fast file transfers.

---

##  Features

- **Local-Only File Sharing** (no internet connection required)
- **QR Code Generation** (scan to recieve directly on phones)  
- **Automatic LAN IP Detection**  
- **Supports all file types** (served as `application/octet-stream`)  
- **Simple CLI Interface** 
- **Runs a temporary local HTTP server** per transfer  

---

## 🦀 Installation

Make sure you have [**Rust** installed](https://www.rust-lang.org/).

## Build project
```bash
cargo build --release
```
## Move the binary to your system's path
Move the compiled binary (`wormhole`) to a directory included in your system’s `PATH`

## Usage
```bash
wormhole <filename>
```
## Example
```bash
wormhole example.pdf
```
## To Recieve
Scan the displayed QR code using your mobile phone camera or any QR scanning app. The file will automatically begin downloading in the browser
