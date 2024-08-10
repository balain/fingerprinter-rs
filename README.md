# Fingerprinter (Rust)

Take fingerprints (md5) of files in specified folder, recursively.

Optionally:
- Compare with previous snapshots and report on any changes (i.e. added, changed, or deleted files).
- Run a continuous watch, cycling every 600sec (configurable).

## Quick Start

Clone the repo and run the file using all defaults. The current folder will be scanned and the output (json) file (`output.json`) will be saved in the `.fingerprint-data` folder:

```bash
$ git clone https://github.com/balain/fingerprinter-rs.git
$ cargo build --release
$ cargo run -- --pathname .
```

## Getting Started

### Prerequisites

The things you need before installing the software:

* Rust

### Installation

To run this program:

```bash
$ git clone https://github.com/balain/fingerprinter-rs.git
```

## Options

Using argparse

```bash
  --pathname PATH   Path to scan (required; no default)
```

## Usage

A few examples of useful commands and/or tasks:

### Use defaults

Scan the current folder:

```bash
$ cargo run -- --pathname .
```

## Sample Output

```bash
❯ cargo run -- --pathname .
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/fingerprinter-rs --pathname .`
"./.github/workflows/release.yml" == b6725cbd0b105218316f9c73866f0ae59e680c411faaf8b1aaaf22e7f9ad7b60
"./.github/workflows/rust.yml" == 213080e2ec9731428d65d49453c39c36d310c24056e5715ae70de743043f3b4e
"./.gitignore" == 260e76b7571b5ab3fd4991c4fdbca3262423e628f872f621a32a64ef06f995fa
"./Cargo.lock" == 42e9e6b8467b07d3e0c3103d59e04ee6592c826b271a489fe6526268b5938633
"./Cargo.toml" == 32eb9657f6079da59b5b9887355753c487dd85ac4d6fbc8db47ec8f36b5e0390
"./README.md" == 2ccd6bfcfda6cc516d5531ad523a29b27dfffb67a811304e29676d040d21c86e
"./src/main.rs" == c29e51e8e2793a593887e7819a65becd7ca9a8c24e429f770b6a1e2de0f42d11
Time elapsed: 1.931917ms
```

## Features

### Implemented
- [x] Initial multithreaded scan (using rayon)

### Future
- [ ] Implement JSON output
- [ ] Improve command line option handling with new options (clap) [following python project options (see below)]
- [ ] Implement watch mode
- [ ] Implement alternative (optional) sqlite storage

## Other Similar or Related Projects

- [fingerprinter-py](https://github.com/balain/fingerprinter-py): Python port (more features but much slower).
- [fingerprinter](https://github.com/balain/fingerprinter): Original Typescript version. No longer maintained.
