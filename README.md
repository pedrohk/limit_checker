---

# 🏦 High-Frequency Risk Limit Checker

A high-performance, real-time risk exposure monitoring engine built in **Rust**.
This system leverages a **Segment Tree with Lazy Propagation** to achieve logarithmic time complexity (**O(log N)**) for both limit checks and exposure updates.

---

## 🎯 Use Case: Real-Time Exposure Management
In High-Frequency Trading (HFT) and Banking, systems must validate thousands of transactions per second against pre-defined credit or risk limits. 

**Scenario:** A bank defines exposure limits for different sectors or groups of assets. When a large trade or a batch of trades happens, the system must:
1.  Verify if the new exposure exceeds the limit of *any* asset in the affected range.
2.  Atomically update the exposure if, and only if, all assets remain within their safety bounds.
3.  Trigger immediate alerts (e.g., Slack) for any blocked transaction to notify risk officers.

---

## ⚖️ Pros and Cons

### Pros
*   **Performance:** $O(\log N)$ complexity for both updates and queries, compared to $O(N)$ for naive array scans.
*   **Atomic Range Validation:** Validates a whole range of accounts/assets in a single pass.
*   **Memory Efficiency:** Flat `Vec`-based tree structure ensures high CPU cache locality and zero-cost abstractions.
*   **Safety:** Rust's ownership model and `Result` types prevent memory corruption and unhandled risk violations.

### Cons
*   **Memory Overhead:** A Segment Tree requires approximately $4N$ space to store the nodes.
*   **Static Bounds:** Standard implementation works best with a fixed number of assets (though Dynamic Segment Trees can mitigate this).
*   **Complexity:** Harder to implement and maintain than simple hash maps or arrays.

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

## 💻 Implementation (Core Engine)

```rust
// Core structure using a flat vector for the Segment Tree
pub struct LimitSegmentTree {
    tree: Vec<Node>,
    n: usize,
}

impl LimitSegmentTree {
    // Updates exposure in range [l, r] with O(log N)
    pub fn update_exposure(&mut self, l: usize, r: usize, val: i64) -> Result<(), String> {
        self.update(1, 0, self.n - 1, l, r, val)
    }

    fn update(&mut self, node: usize, start: usize, end: usize, l: usize, r: usize, val: i64) -> Result<(), String> {
        // ... Segment Tree Logic with Lazy Propagation ...
        if start >= l && end <= r {
            if self.tree[node].current_exposure + val > self.tree[node].max_limit {
                return Err(format!("Limit violation at range {}-{}", start, end));
            }
            self.apply(node, val);
            return Ok(());
        }
        // ... push_down and recursive update ...
    }
}
```

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
