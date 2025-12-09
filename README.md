# 🔐 Rust Password Tool

A secure password generator and password validation tool written in Rust.

This project is a learning-focused CLI application designed to demonstrate:
- Secure random generation
- Password policy enforcement
- Modular Rust architecture

---

## ✨ Features

### ✅ Password Generator
- Cryptographically secure random generation (`OsRng`)
- Configurable length
- Customizable character sets:
  - Lowercase
  - Uppercase
  - Digits
  - Symbols

### ✅ Password Validator
Industry-inspired validation rules:
- Minimum length: **12 characters**
- Must contain at least:
  - One lowercase character
  - **3 out of 4 character categories** (uppercase, digit, symbol)

### ⏳ Under Development
- Password strength scoring
- Entropy-based rating
- CLI usability improvements

---

## 🧩 Project Goals

This project was created to:

- Learn and practice **Rust fundamentals**
- Build a real-world **security-oriented CLI tool**
- Apply clean architecture patterns (layered design)

---

## 🚀 Usage

Run the program and select a mode:

```text
1. Password generator
2. Password validator
```

## 🗺 Roadmap
- [x] Core password generator
- [x] Configurable options
- [x] Password policy validator
- [ ] Strength scoring

## 🛠 Tech

- Rust
- `rand` crate (OS-backed RNG)
- Modular architecture (`cli`, `services`, `models`, `utils`)

## 📌 Disclaimer
This project is a learning tool and does not aim to fully implement all requirements of NIST SP 800-63B or OWASP ASVS.