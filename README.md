# metis-rs

### Lightweight Rust wrapper for graph partitioning using METIS

#### NOTE: **This is still in the very early stages of development, so expect API changes, improvements etc.**

Internally uses this CMake enhanced METIS library:
https://github.com/scivision/METIS which uses https://github.com/KarypisLab/METIS and https://github.com/KarypisLab/GKlib respectively.

#### Compatibility
- Requires Rust, CMake and a C99 compiler (GCC, Clang, MSVC, ...)
- **Tested on:**
  - Windows (11) with MSVC
  - Linux (ubuntu) with Clang
  - MacOS (13.1 arm64)