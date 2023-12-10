# File Encryptor

A simple file encryption tool written in Rust, utilizing ChaCha20Poly1305 for key generation and AES256 for file encryption. This tool allows you to encrypt all files within a specified directory using a generated key.

## Installation

1. Clone the repository:

```bash
git clone https://github.com/CTFcrozone/RustCryptor.git
cd RustCryptor
```
2. Build the project:

```bash
cargo build --release
```
3. Run the executable

```bash
.\target\release\cryptor.exe
```

## Usage
```bash
Enter a path to encrypt:
C:\absolute\path\to\directory
```
