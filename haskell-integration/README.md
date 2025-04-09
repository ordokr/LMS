# LMS Haskell Integration

This module provides high-performance Haskell components for the Learning Management System (LMS) platform.

## Features

- **Concurrent CRDT-based sync engine**
- **Formal verification of blockchain operations**
- **Query optimization for educational data**
- **Domain-specific languages for course requirements and queries**
- **Hybrid Haskell-Rust integration using FFI and zero-copy data exchange**
- **Performance profiling with Windows-compatible scripts**

## Setup

### Prerequisites

- GHC 9.2 or later
- Cabal 3.6 or later
- Alex (lexer generator)
- Happy (parser generator)

### Installation

Run the setup script to install required tools:

```bash
./tools/setup_parsers.bat
```

### Building

```bash
make build
```

### Testing

```bash
make test
```

## Domain-Specific Languages

### Course Requirement DSL

Expressive language for defining course completion requirements:

```
complete assignment-101
score above 70% final-exam
and {
  complete assignment-101,
  score above 60% midterm
}
or {
  complete all modules,
  minimum 5 posts
}
```

### Query Language

Optimized SQL-like query language for educational data:

```sql
select s.name, a.title, s.score 
from submissions s join assignments a
where s.score > 70.0 and a.due_date < '2023-01-01'
```

## Integration with Rust/Tauri

Haskell components are compiled to static libraries and loaded by the Rust application through FFI.  
See `cbits/lms_bridge.h` for the exposed C API.

## Profiling & Performance

Use the provided scripts to profile both Haskell and Rust components. For example, run:

```bash
./tools/profile_hybrid.ps1
```

(for Windows using PowerShell) or

```batch
profile_hybrid.bat
```

(for a simpler Windows batch approach).

## Development

Run benchmarks to ensure performance optimization:

```bash
make bench
```

For parser development, modify the `.x` (Alex) or `.y` (Happy) files in `src/Parser` and regenerate using:

```bash
make parsers
```

## Integration History & Changelog

- **April 2025:**  
  - Integrated a performance-first Haskell CRDT engine and formal verification for blockchain operations.
  - Added domain-specific languages (DSL) for course requirements and query optimization.
  - Established a hybrid integration approach where Haskell is tightly coupled with Rust using zero-copy FFI.
  - Introduced comprehensive profiling scripts, including Windows-compatible PowerShell and batch versions.
  - Updated the project documentation, including the Makefile and Stack/Cabal configurations, to reflect these changes.

- **Ongoing:**  
  - Further refinements will address memory management improvements and enhanced cross-language performance analysis.

## License

This project is licensed under the MIT License.