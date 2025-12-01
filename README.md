# aoc

My solutions for Advent of Code.

## Structure

Each ``<Year>`` has a ``<Lang>`` folder/project. The structure of the folder depends on the language
used, but must not require any dependencies from outside.

### Rust

In Rust the best setup is to use a workspace, with one or more shared library crates and binary crates for each individual day.

## Minimal Requirements for projects

- Cli program that supports:
  - running a single day
  - running a selection of days
  - running all days
  - measure performance for all days or selection or single days (in some way)

e.g. for running day 25:

```bash
  main 25
```

## Languages of interest to me (in no particular order)

- Rust
- C++
- Haskell
- Kotlin
- C#
- Python
- Typescript
- Clojure

### Rust Guide

TODO describe what to do to setup Rust per year, because obviously I don't remember...
