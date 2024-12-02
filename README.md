# ⚡ Graft - High-Performance Graph Analysis Framework

**Graft** is a graph analysis framework written in **Rust**, designed for high performance and safe concurrency. It provides implementations of classic graph algorithms with modern optimizations.

## 🌟 Key Features

- 🚀 **High Performance** with built-in concurrency
- 🔒 **Thread-safe** using concurrent data structures
- 📊 **Classic Graph Algorithms**:
  - Minimum Spanning Tree (Prim's Algorithm)
  - Maximum Flow (Ford-Fulkerson Algorithm)
- 🧮 Powerful **mathematical expression parser**
- 💾 Efficient **caching system**
- 🔄 **Event-driven architecture**

## 🛠️ Project Structure

```plaintext
src/
├── core/
│   ├── mod.rs       # Core system functionality
│   └── parser.rs    # Mathematical expression parser
└── algorithms/
    └── graph.rs     # Graph algorithms implementation
```

## 🚀 Technologies Used

- **Concurrency**: `parking_lot`, `crossbeam`, `rayon`
- **Data Structures**: `petgraph`, `dashmap`
- **Numerics**: `num-complex`
- **Error Handling**: `thiserror`

## 📥 Installation

Add Graft to your Rust project with:

```bash
cargo add graft
```

## 💡 Basic Usage Example

```rust
use graft::algorithms::Grf;

fn main() {
    // Initialize a new graph
    let mut graph = Grf::new();
    
    // Add nodes
    graph.add_nd(1, 1.0, 0.0, 0.0);
    graph.add_nd(2, 2.0, 1.0, 1.0);
    
    // Add an edge
    graph.add_ed(1, 2, 5.0);
    
    // Calculate Minimum Spanning Tree (MST)
    let mst = graph.mst();
    println!("MST edges: {:?}", mst);
}
```

### ⚙️ Concurrency Example

```rust
use graft::core::Core;

fn main() {
    let core = Core::new(4); // Initialize with 4 worker threads
    core.start();
    
    // Perform concurrent operations here
    
    core.stop();
}
```

## ⚡ Performance

**Graft** is optimized for:
- Efficient memory usage
- Safe, parallel concurrency
- High throughput for graph operations

## 🤝 Contribution

We welcome contributions! Follow these steps to contribute:

1. **Fork** the repository.
2. Create a new feature branch (`git checkout -b feature/new-feature`).
3. Commit your changes (`git commit -m 'Add new feature'`).
4. Push to the branch (`git push origin feature/new-feature`).
5. Open a **Pull Request**.

## 📜 License

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.

## 📧 Contact

**Sazumi Viki** - [@moe.sazumiviki](https://instagram.com/moe.sazumiviki)

🔗 Project Link: [https://github.com/sazumivicky/graft](https://github.com/sazumivicky/graft)

## 🙌 Acknowledgements

Special thanks to these amazing projects:
- [petgraph](https://github.com/petgraph/petgraph)
- [parking_lot](https://github.com/Amanieu/parking_lot)
- [crossbeam](https://github.com/crossbeam-rs/crossbeam)
