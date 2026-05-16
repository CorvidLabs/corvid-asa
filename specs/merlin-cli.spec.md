---
module: merlin-cli
version: 1
status: draft
files:
  - merlin-cli/src
depends_on: []
---

## Purpose

This module provides a CLI tool for generating the Corvid ASA Holder Role Tiers & Perks website.

## Files

- `merlin-cli/src/main.rs` - Main CLI entry point
- `merlin-cli/src/lib.rs` - Library functions

## Public API

- `hello()` - Returns a greeting string

## Invariants

- The CLI should always output a greeting when run
- The library function should always return the same string

## Behavioral Examples

```bash
$ cargo run
Hello, Corvid ASA!
```

## Error Cases

- If the CLI fails to compile, it should show a compilation error
- If the library function is changed, tests should fail

## Dependencies

- Rust 2021 edition
- Standard library only

## Change Log

- v1: Initial version