# aoc

My solutions for Advent of Code.

## Structure

Each ``<Year>`` has a ``<Lang>`` folder/project. The structure of the folder depends on the language
used, but must not require any dependencies from outside.

### Rust

In Rust the best setup is to use a workspace, with one or more shared library crates and
binary crates for each individual day.

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

## Setup aoc-cli

### Update Session Cookie

1. login to aoc with github on firefox
1. right-click -> inspect
1. go to storage tab -> cookies
1. there it is
1. copy the value and put it under `C:\Users\<user>\.adventofcode.session`

### How to use aoc-cli

#### For the current day

`aoc d -Io`

#### For any specific day

`aoc d -Iod <day>`

#### Submitting

`aoc s -d <day> <part> <answer>`

## Rust Guide

### cargo-generate

I have this nice other github project `cargo-aoc-template` that can be used with cargo generate.

TODO describe in more detail
