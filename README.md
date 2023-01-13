# aoc

Solutions for Advent of Code

## What was wrong with AdventOfCode Repository

The core idea to have a structure like this:

- ``AoC_<Year>`` contains the entries:
  - ``Day<dd>_<Name>_<Lang>`` which always is its own project!

was flawed.

Its main downside is that the individual projects might use some 3rd party libraries
(regex, anyhow, lazy_static to just name a few for Rust) and those can consume a lot
of memory when all the individual projects are compiled.

## Structure

- ``<Year>`` contains:
  - ``<Lang>`` contains:
    - ``<main>.<ext>`` the main entry point for execution
    - ``day<dd>`` folder or module for the individual days used by main (exact structure might be language dependent, let's see)

## Minimal Requirements for main

- Cli program that supports:
  - running a single day
  - running a selection of days
  - running all days
  - measure performance for all days or selection or single days (in some way)

e.g.

```bash
main 25 <
```

runs day 25

## Focus on language

- Rust
- C++
- Haskell
