# Fingerprinter (Rust)

Take fingerprints (sha256) of files in specified folder, recursively.

## Quick Start

Clone the repo, build the binary, and run it using all defaults. The current folder will be scanned and the output report will be shown on screen:

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

### Sample Runtime (on Apple Studio M1)

Run against ~14.4k files in just over 3sec (~4.8k files per second).

```bash
❯ cargo run --release -- --pathname ..|wc -l
    Finished `release` profile [optimized] target(s) in 0.03s
     Running `target/release/fingerprinter-rs --pathname ..`
Time elapsed: 3.102094667s
   14402
```

Run against ~344k files in about 34.5sec (~10k files per second).

```bash
❯ cargo run --release -- --pathname ../..|wc -l
    Finished `release` profile [optimized] target(s) in 0.03s
     Running `target/release/fingerprinter-rs --pathname ../..`
Time elapsed: 34.518055958s
  343715
```

## Features

### Implemented
- [x] Calculate sha256 checksums for files/folders recursively
- [x] Multithreaded scan (using rayon)

### Future
Optionally:
- [ ] Implement JSON output
- [ ] Compare with previous snapshots and report on any changes (i.e. added, changed, or deleted files).
- [ ] Run a continuous watch, cycling every 600sec (configurable)
- [ ] Improve command line option handling with new options (clap) [following python project options (see below)]
- [ ] Implement alternative (optional) sqlite storage
- [ ] Optimize performance

## Other Similar or Related Projects

- [fingerprinter-py](https://github.com/balain/fingerprinter-py): Python port (uses md5; more features but much (!) slower).
- [fingerprinter](https://github.com/balain/fingerprinter): Original Typescript version. No longer maintained.

## License

https://mit-license.org

MIT, Copyright &copy; 2024, John D. Lewis

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.