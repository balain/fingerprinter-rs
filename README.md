# Fingerprinter (Rust)

Take fingerprints (xxhash) of files in specified folder, recursively.

## Quick Start

Clone the repo, build the binary, and run it using all defaults. The current folder will be scanned and the output report will be shown on screen:

```bash
$ git clone https://github.com/balain/fingerprinter-rs.git
$ cargo build --release
$ cargo run -- --pathname .
```

## Getting Started

### Prerequisites and Dependencies

The things you need before installing the software:

* Rust
* xxhash_rust: For hashing
* chrono: For timestamps

### Installation

To run this program:

```bash
$ git clone https://github.com/balain/fingerprinter-rs.git
$ cargo add xxhash_rust
```

## Options

```bash
  -p, --path <PATH>      Path to process
  -o, --output <OUTPUT>  [default: ./output.json]
  -s, --save-full-path   -- not yet implemented  
  -h, --help             Print help
  -V, --version          Print version
```

- Configure for Sha256 or Xxhash [Default: Xxhash]

## Usage

A few examples of useful commands and/or tasks:

### Use defaults

Scan the current folder:

```bash
$ cargo run -- --path .
```

## Sample Output

### SHA256
```bash
❯ cargo run --release -- --path .
    Finished `release` profile [optimized] target(s) in 0.02s
     Running `target/release/fingerprinter-rs --path .`
"./.github/workflows/release.yml" == b6725cbd0b105218316f9c73866f0ae59e680c411faaf8b1aaaf22e7f9ad7b60
"./.github/workflows/rust.yml" == 213080e2ec9731428d65d49453c39c36d310c24056e5715ae70de743043f3b4e
"./.gitignore" == 6317d3e78e7a71916a797e058022ab540c8fc0f4373b56501dcf01c168cf0c21
"./Cargo.lock" == e206393b943e11d95907b2d3a114af556d8a06cbaaa1c216bf28335a48759fce
"./Cargo.toml" == cfbc3fe391da7f5de57afb800d9d68ae864a98b92a3b83991fea66f5672e89d0
"./LICENSE.md" == 3373b2f94fe100229792b388983ae2a3a990d33308b9c23db06fafe88efde607
"./README.md" == 3628ad01e53d3a02685f045f99d9da60b64d3182399a55fafcbd9e34afb84039
"./acc-recordings.txt" == f14eec8d17e6b07178e91b03304e9b78cf8e051d473f6ede1b0bc6b89821d800
"./src/main.rs" == 850097b0fdc5e0eb155cb744faff2c3d8d70c6e6bad2b7c4d4f4a976f87ac71a
Time elapsed: 1.181708ms
```

### Xxhash
```bash
❯ cargo run --release -- --path .
    Finished `release` profile [optimized] target(s) in 0.02s
     Running `target/release/fingerprinter-rs --path .`
"./.github/workflows/release.yml" == fa0549909fe77a4f
"./.github/workflows/rust.yml" == a4b34512721a82d8
"./.gitignore" == 122105a1456414a4
"./Cargo.lock" == 71ac41962788727e
"./Cargo.toml" == 7658b6aaab2c0055
"./LICENSE.md" == 41f4b20423632cb3
"./README.md" == 73b4b99518797dd7
"./acc-recordings.txt" == 64fd56b86a83af1f
"./src/main.rs" == 3d3356a74df556a4
Time elapsed: 713.084µs
```

### Sample Runtime (on Apple Studio M1)

Run against ~14.4k files in just over 3sec (~4.8k files per second).

#### SHA256
```bash
❯ cargo run --release -- --path ..|wc -l
    Finished `release` profile [optimized] target(s) in 0.02s
     Running `target/release/fingerprinter-rs --path ..`
Time elapsed: 4.319165708s
   18085
```

- SHA256: Run against ~347k files in about 31.1sec

```bash
❯ cargo run --release -- --path ../..|wc -l
    Finished `release` profile [optimized] target(s) in 0.02s
     Running `target/release/fingerprinter-rs --path ../..`
Time elapsed: 31.108233334s
  347400
```

#### Xxhash

Typically 2-5x faster on non-trivial runs

```bash
❯ cargo run --release -- --path ..|wc -l
    Finished `release` profile [optimized] target(s) in 0.10s
     Running `target/release/fingerprinter-rs --path ..`
Time elapsed: 844.374542ms
   18085
```

- Xxhash: Run against ~347k files in about 16.8sec

```bash
❯ cargo run --release -- --path ../..|wc -l
    Finished `release` profile [optimized] target(s) in 0.02s
     Running `target/release/fingerprinter-rs --path ../..`
Time elapsed: 16.772243416s
  347400
```

## Features

### Implemented
- [x] Calculate sha256 checksums for files/folders recursively
- [x] Multithreaded scan (using rayon)
- [x] Switched to xxhash_rust (instead of sha256) - significant speed improvement
- [x] JSON output
- [x] Improve command line option handling with new options (clap) [following python project options (see below)]

### Future (in priority order)
- [ ] Compare with previous snapshots and report on any changes (i.e. added, changed, or deleted files).
- [ ] Sqlite storage
- [ ] Watch mode with configurable period
- [ ] Use [crossterm](https://docs.rs/crossterm/latest/crossterm/) for watch mode

Optionally:
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