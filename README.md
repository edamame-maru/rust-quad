# rust-quad
Simple quadratic equation implementation in rust.
## Project layout
```
.
├── Cargo.lock
├── Cargo.toml
├── README.md
└── src
    └── main.rs

2 directories, 4 files
```
## Installation
### Installing the Rust toolchain
If you already have rust installed on your system, you can skip this step. To check if Rust is installed, on type: 
```
rustup --version
```
If rust is installed, you should see the output:
```
rustc x.y.z (abcabcabc yyyy-mm-dd)
```
to indicate the version of Rust installed.
#### macOS, Linux
If you're running macOS, Linux, or another Unix-like OS, run the following command to download Rustup and install Rust. Follow the on-screen instructions, and, when prompted, just click the enter key.

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
You may need to restart your terminal session to refresh your PATH and other environment variables.

#### Windows

If you're running Windows, download and run the installer, [rustup-init.exe](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe) and then follow the on-screen instructions. You may need to restart your terminal session to refresh your PATH and other environment variables.

#### Standalone installers and Source code
If you want to build the Rust toolchain from source code, want an offline installation, prefer a more platform-integrated, graphicall installer on Windows, or want to install Rust on a different system, visit [this official website](https://forge.rust-lang.org/infra/other-installation-methods.html), or [the official landing page](https://www.rust-lang.org/tools/install).

### Getting the source code
Clone this repository or download the [zip file](https://github.com/edamame-maru/rust-quad/archive/refs/heads/main.zip). If you downloaded the zip file, make sure you extract the archive. Then, in the project folder, ```rust-quad```, run:
```
cargo build --release
```
which should produce the resultant binary in ```/path/to/rust-quad/target/release/```. 
Furthermore, to add the compiled binary as a command, type: 
```
cargo install --path .
```
and then run the application anywhere with: 
```
rust-quad
```
