# rsdp
A lightweight and async API service built with Rust, Actix-Web, and Tokio that reads CSV files from the command line or via file upload, transforms the data into JSON, and serves it over an HTTP endpoint.

# 📊 CSV to JSON API Service in Rust

This project is a simple API service built using **Rust**, **Actix-Web**, and **Tokio**. It allows you to:

- 🧾 Read a CSV file passed via CLI and expose it as a JSON API
- 📤 Upload a CSV file via HTTP POST request
- 🔄 Automatically transform CSV data into JSON format
- 🌐 Serve the JSON through an HTTP API (`/data`)

---

## 🚀 Features

- 🧵 Async and concurrent with `tokio`
- 🌐 HTTP server using `actix-web`
- 📦 Multipart file upload support
- 📑 CSV parsing using `csv` and serialization with `serde`
- 🧠 Shared state with live in-memory JSON data

---

## 🛠 Requirements

- Rust (latest stable)
- Cargo

---

## 📦 Installation

```bash
git clone https://github.com/mcadriel/rsdp.git
cd rsdp
cargo build --release