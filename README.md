# Junky

<img src="assets/icon.svg" width="72" alt="Junky logo">

Safe, scan-first junk cleaner for Windows, written in Rust. It targets known
junk locations instead of sweeping the whole drive by extension.

## Desktop app

```console
cargo run --release --bin junky-gui
```

Dark dashboard with a category sidebar (safe always visible, aggressive
behind a switch), reclaimable-size hero stats, per-file selection, an age
filter, background scanning with a progress state, and a confirm dialog
before anything is deleted. The same engine also ships as a CLI (below).

## Safety model

- Targeted roots only: Temp folders, Update cache, caches, error reports.
- Extension filters apply inside those roots, never across the whole drive.
- Blocklist never touches System32, WinSxS, Installer, or Package Cache.
- Symlinks are never followed. Locked files are skipped, not forced.
- `scan` previews everything. `clean` refuses to run without `--yes`.
- Aggressive cleaners (Prefetch, previous Windows installs, stray-extension
  sweep) are off unless `--include-aggressive` is passed.

## Safe cleaners (default)

user-temp, system-temp, update-cache, delivery-optimization, recycle-bin,
thumbnail-cache, icon-cache, internet-cache, error-reports, crash-dumps,
shader-cache, office-cache, app-caches, defender-history, windows-logs,
browser-edge, browser-chrome, browser-firefox, browser-brave.

## Aggressive cleaners (opt-in)

prefetch, windows-old, extension-sweep, font-cache.

## Never touched

WinSxS component store (use `DISM /Online /Cleanup-Image
/StartComponentCleanup` instead), Windows Installer, Package Cache, update
database (`SoftwareDistribution/DataStore`), signature catalogs (catroot2),
Office install cache (MSOCache), the Recovery image, and the Search index.
Only the contents of `SoftwareDistribution/Download` are cleaned, after
updates are installed.

## Usage (CLI)

```console
junky list
junky scan
junky scan --include-aggressive --older-than-hours 24 --limit 20
junky scan --json
junky clean --yes
junky clean --yes --include-aggressive --json
```

## Build the exes

On Windows:

```console
cargo build --release --bins
```

This produces `target/release/junky.exe` (CLI) and
`target/release/junky-gui.exe` (desktop app, no console window). CI builds
and uploads both on every push. For local development on macOS/Linux, point
the Windows roots at a fixture tree:

```console
JUNKY_TEST_ROOT=/tmp/jc-test junky scan
```
