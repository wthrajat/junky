# Developing Junky

## Prerequisites

- Rust stable via [rustup](https://rustup.rs)
- Windows to build the `.exe` files; macOS/Linux work for development using
  a fixture tree (see below)

## Everyday commands

```console
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release --bins
cargo run --release --bin junky-gui
cargo run -- -- list
```

## Testing without Windows

Point the Windows roots at a fixture tree. Any `C:/...` path resolves
under `JUNKY_TEST_ROOT`, missing roots are skipped:

```console
mkdir -p /tmp/jc-test/Users/Test/AppData/Local/Temp
export JUNKY_TEST_ROOT=/tmp/jc-test
export TEMP="C:\Users\Test\AppData\Local\Temp"
export TMP="C:\Users\Test\AppData\Local\Temp"
export LOCALAPPDATA="C:\Users\Test\AppData\Local"
export APPDATA="C:\Users\Test\AppData\Roaming"
export USERPROFILE="C:\Users\Test"
export ProgramData="C:\ProgramData"
export SystemRoot="C:\Windows"
export SystemDrive="C:"
cargo run -- scan
```

`cargo test` covers size formatting, the safety blocklist, and the UI
adapters. `JUNKY_ASSUME_ELEVATED=1` simulates an elevated process.

## Project layout

```text
ui/                  Slint markup (app shell + reusable components)
src/
  main.rs            CLI entry point (thin)
  bin/junky-gui.rs   Desktop entry point (thin)
  lib.rs             Crate root shared by both binaries
  cli/               Argument parsing, command dispatch, output views
  core/              Risk levels, scan/clean summaries, age gates, formatting
  platform/          Windows path expansion, drive listing, elevation check
  safety/            Blocklist of never-touch locations, file age rules
  engine/            Parallel file walker, scan fan-out, file remover
  cleaners/          One module per cleaner plus a registering catalog
  ui/                Slint bridge: pure adapters + window wiring
assets/              icon.svg master, icon-64.png, icon.ico
scripts/             Dev tooling (icon generation)
```

## Adding a cleaner

1. Create `src/cleaners/my_area.rs` with a unit struct implementing the
   `Cleaner` trait (`id`, `display_name`, `risk`, `needs_admin`, `scan`).
   Resolve roots with `platform::environment::existing_paths` and collect
   with `engine::file_walker`, never with a hand-rolled walk.
2. Declare the module in `src/cleaners/mod.rs`.
3. Register it in `src/cleaners/catalog.rs`.
4. Run `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test`.

Pick `Risk::Safe` only when deletion is pure cache or logs that Windows
rebuilds. Anything else is `Risk::Aggressive` and stays behind the opt-in
flag. Never add roots under System32, WinSxS, Installer, Package Cache,
`SoftwareDistribution/DataStore`, catroot2, MSOCache, or Recovery; extend
`safety::blocklist` instead if a new never-touch location appears.

## Icons

`assets/icon.svg` is the master. PNG and ICO are generated from it:

```console
python3 -m venv /tmp/iconenv
/tmp/iconenv/bin/pip install pillow
/tmp/iconenv/bin/python scripts/gen_icons.py
```

## Releases

1. Bump `version` in `Cargo.toml` and commit it.
2. Tag the release: `git tag -s v0.1.0 -m "v0.1.0"` (match the version).
3. Push the tag: `git push origin v0.1.0`.
4. `.github/workflows/release.yml` builds both exes on Windows and
   attaches them to the GitHub Release automatically.

Unsigned builds trigger SmartScreen; users pick More info, then Run
anyway. A code-signing certificate removes that prompt.

CI (`.github/workflows/rust.yml`) runs fmt, clippy, tests, and a release
build on Windows, uploading both exes as artifacts.

## Conventions

Small focused modules, one concept per file, descriptive names. No code
comments unless omitting them would hide a real trap. Readability wins.
