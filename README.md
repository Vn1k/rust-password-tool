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

### 🛡 Password Validator
Follows common industry password rules:
- Minimum length: **12 characters**
- Must include lowercase
- Must include 3 out of 4 categories:
  - Uppercase
  - Digit
  - Symbol
  - (Lowercase always required)

Validator provides clear feedback on failing rules.

---

## ⚙ Configuration System
`rustpass config`

Allows updating:
- default length
- allowed character sets
- all changes persisted to config file

## 🧰 CLI Commands (Clap-powered)

- `rustpass` → generate password using saved config  
- `rustpass validate <value>` → validate password  
- `rustpass config` → interactive configuration menu  

---

## ⏳ Planned Features
- Password strength scoring  
- Entropy-based strength rating  

---

## 🧩 Project Goals

This project was created to:

- Learn and practice Rust fundamentals  
- Explore cryptographic randomness in practical applications  
- Build a real-world security-focused CLI tool  
- Implement a clean, layered architecture:
  - cli/ → user interaction
  - services/ → business logic
  - models/ → core data structures
  - config/ → constants + config file
  - utils/ → helper utilities

---

## 🚀 Usage

### Generate password (default)
`rustpass`
### Validate a password
`rustpass validate "<value>"`
### Configure settings
`rustpass config`

---

## 🗺 Roadmap
- [x] Core password generator  
- [x] CLI-based configurable settings  
- [x] Password policy validator  
- [ ] Strength scoring system  
- [ ] Entropy calculation  

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