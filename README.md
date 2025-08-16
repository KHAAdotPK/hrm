## Hierarchical Reasoning Model (HRM) in Rust

The **Hierarchical Reasoning Model (HRM)** is a Rust-based implementation of a recurrent neural network with hierarchical modules, inspired by the architecture described in [arXiv:2506.21734](https://arxiv.org/abs/2506.21734). The model features a High Level Module (HLM) and a Low-Level Module (LLM) designed for sequential reasoning tasks. This project provides the foundational components for training and evaluating the HRM, including a parameter management system, synthetic data generation, and command line argument handling.

This repository is a work-in-progress implementation, focusing on modular and reusable components to jumpstart the development of the full HRM model. The code is written in Rust for performance and safety, with dependencies on custom and external libraries.

```
hrm/
├── implementation/           # HRM library crate
│   ├── src/
│   │   ├── constants.rs      # Model hyperparameters and constants
│   │   ├── hrm.rs            # HRM model definition (placeholder)
│   │   ├── lib.rs            # Library module declarations
│   │   ├── parameter.rs      # Parameter management with doubly-linked list
│   │   ├── utility.rs        # Synthetic data generation for training/validation
│   └── Cargo.toml            # HRM library dependencies
├── lib/                      # External library dependencies
│   ├── numrs/                # Custom numerical computing library
│   ├── argsv-rust/           # Command-line argument parsing library
│   └── PULL.cmd              # Script to clone dependency repositories
├── src/
│   └── main.rs               # Application entry point for HRM usage
└── Cargo.toml                # Wrapper application dependencies
```

### Dependencies

- **Rust Dependencies** (specified in `Cargo.toml`):
  - `hrm`: The core HRM library (local path: `./implementation`).
  - `numrs`: Custom numerical computing library for tensor operations (local path: `./lib/numrs`).
  - `argsv-rust`: Command-line argument parsing library (local path: `./lib/argsv-rust/lib/rust/argsv`).
  - `rand = "0.8.4"`: For random number generation in synthetic data creation.
- **C++ Dependencies** (cloned via `PULL.cmd`, not directly used in Rust):
  - `allocator`, `parser`, `String`, `csv`, `ala_exception`: Custom C++ libraries for potential future integration.

### Installation

1. **Clone the Repository**:
```bash
git clone https://github.com/KHAAdotPK/hrm.git
cd hrm
```

2. **Clone Dependencies**:
```bash
cd lib 
./PULL.cmd
...

### **Contributing**

Contributions are welcome!  
Feel free to open an issue or submit a pull request if you'd like to improve or expand the project.

### **License**

This project is governed by a license, available in the accompanying `LICENSE` file.  
Please refer to it for complete licensing details.


