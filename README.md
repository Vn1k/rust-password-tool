# 🔐 Rust Password Tool - (rustpass)

A secure, configurable password generator and validation tool written in Rust.
Designed as a learning-oriented CLI project focused on security, clean architecture, and Rust fundamentals.

---

## ✨ Features

### 🔑 Password Generator
- Uses cryptographically secure randomness via `OsRng`
- Fully configurable:
  - Length (default from config file)
  - Lowercase
  - Uppercase
  - Digits
  - Symbols
- Configuration is stored in `~/.config/rustpass/config.toml`

### 🛡 Password Validator & Scoring
Follows common industry password rules with entropy calculation:
- **Smart Validation:**
  - Minimum length: **12 characters**
  - Must include lowercase
  - Must include 3 out of 4 categories (Uppercase, Digit, Symbol, Lowercase)
- **Entropy Calculation:** Calculates information entropy (bits) to measure theoretical strength.
- **Strength Scoring:** Categorizes password into Poor, Weak, Good, or Excellent.

---

## 🛠 Installation & Usage

Ensure you have Rust and Cargo installed.

```bash
# Clone the repository
git clone [https://github.com/vinik/rustpass.git](https://github.com/vinik/rustpass.git)
cd rustpass

# Option 1: Install globally (Recommended)
# This allows you to run 'rustpass' from anywhere in your terminal
cargo install --path .

# Option 2: Run directly without installing
cargo run --release

# Option 3: Build binary manually
cargo build --release
./target/release/rustpass
```

### Commands
- Generate Password (uses default config):
```bash
rustpass
```
- Validate a Password (Check strength & entropy):
```bash
rustpass validate
```
- Configure Settings:
```bash
rustpass config
```

## ⚙ Configuration System
Allows updating default length and allowed character sets. All changes are persisted to the config file using Serde and toml.

## 🧩 Project Goals

This project was created to:

- Learn and practice Rust fundamentals  
- Explore cryptographic randomness in practical applications  
- Build a real-world security-focused CLI tool  
- Implement a clean, layered architecture:
  - `cli/` → user interaction
  - `services/` → business logic
  - `models/` → core data structures
  - `config/` → constants + config file
  - `utils/` → helper utilities

---

## 🗺 Roadmap
- [x] Core password generator  
- [x] CLI-based configurable settings  
- [x] Password policy validator  
- [x] Strength scoring system  
- [x] Entropy calculation  

---

## 🛠 Tech Stack

- Rust  
- rand — cryptographically secure RNG (OsRng)  
- clap — CLI interface & subcommands  
- serde + toml — config serialization  
- directories — cross-platform config location  

---

## 📌 Disclaimer

This tool is a learning project and does not fully implement requirements of NIST SP 800-63B, OWASP ASVS, or other formal security standards.