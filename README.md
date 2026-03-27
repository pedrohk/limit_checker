---

# 🏦 High-Frequency Risk Limit Checker

A high-performance, real-time risk exposure monitoring engine built in **Rust**.
This system leverages a **Segment Tree with Lazy Propagation** to achieve logarithmic time complexity (**O(log N)**) for both limit checks and exposure updates.

---

## 🚀 The Technical Challenge

In financial institutions, **credit limit and exposure monitoring must be near-instantaneous**.
Traditional relational database approaches often suffer from:

* I/O latency
* Lock contention
* Poor scalability under high-frequency trading (HFT) workloads

This project addresses these challenges by maintaining an optimized **in-memory risk structure**:

* **Segment Tree**
  Enables efficient range queries and updates (e.g., assets within a sector)

* **Lazy Propagation**
  Defers updates to child nodes until necessary, reducing CPU overhead

* **Transactional Integrity**
  Uses Rust’s `Result` type to guarantee that invalid transactions are rejected *before* state mutation

---

## 🛠️ Tech Stack

* **Rust (2021 Edition)** – Memory safety without garbage collection + native performance
* **Tokio** – Asynchronous runtime for non-blocking operations
* **Reqwest** – Async HTTP client for Slack webhook integration
* **Proptest** – Property-based testing for large-scale edge case validation

---

## 📦 Key Features

* ✅ **Atomic Range Updates**
  Update exposure across multiple assets/accounts simultaneously

* ✅ **Violation Guard**
  Rejects the entire operation if any limit is exceeded

* ✅ **Slack Integration**
  Real-time alerts via incoming webhooks

* ✅ **Cache-Friendly Design**
  Flat `Vec`-based tree structure optimized for CPU cache locality

---

## 🔧 Getting Started

### Prerequisites

* [Rust & Cargo](https://rustup.rs)
* Windows users: C++ Build Tools (MSVC) or MinGW

---

### Installation

```bash
git clone https://github.com/pedrohk/limit_checker.git
cd limit_checker
```

1. Configure your Slack Webhook URL in `src/main.rs`
2. Build and run:

```bash
cargo run
```

---

## 🧪 Running Tests

Run unit tests and property-based stress tests:

```bash
cargo test
```

---

## 📈 Usage Example

```rust
// Initialize 5 assets with specific limits
let initial_limits = vec![1000, 2000, 1500, 3000, 5000];
let mut monitor = LimitSegmentTree::new(initial_limits);

// Attempt to add exposure to a range [0, 2]
match monitor.update_exposure(0, 2, 800) {
    Ok(_) => println!("Transaction Approved"),
    Err(e) => println!("🚨 Risk Alert: {}", e),
}
```

---
