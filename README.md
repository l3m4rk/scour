# scour

![CI](https://github.com/l3m4rk/scour/actions/workflows/ci.yml/badge.svg)

A grep-like search tool written in Rust.

## Features

### Implemented
- Basic pattern search
- `-i` case insensitive search
- `-n` show line numbers
- stdin support

### Planned
- `-r` recursive directory search
- `-v` inverted search (lines that do NOT match)
- `-c` count matches
- `-l` print only filenames
- `-w` whole word match
- `-E` extended regex
- colored output (highlight matches)
- filename in output when searching multiple files

## Example

```bash
cargo run -- "fn" srs/main.rs
```

## Development

```bash
cargo test
```
