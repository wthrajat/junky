<div align="center">
  <img src="assets/icon.svg" width="96" alt="Junky logo">
  <h1>Junky</h1>
  <p><strong>Reclaim gigabytes of Windows disk space — safely.</strong></p>
  <p>Scan-first junk cleaner with a calm native UI.<br>No guesswork. Nothing deleted without your say-so.</p>
  <p>
    <a href="https://github.com/wthrajat/junky/releases/latest"><img src="https://img.shields.io/github/v/release/wthrajat/junky" alt="Latest release"></a>
    <a href="https://github.com/wthrajat/junky/releases"><img src="https://img.shields.io/github/downloads/wthrajat/junky/total" alt="Total downloads"></a>
    <a href="https://github.com/wthrajat/junky/actions/workflows/rust.yml"><img src="https://github.com/wthrajat/junky/actions/workflows/rust.yml/badge.svg" alt="Build status"></a>
    <a href="LICENSE"><img src="https://img.shields.io/github/license/wthrajat/junky" alt="License: MIT"></a>
    <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/Made_with-Rust-ce422b?logo=rust&logoColor=white" alt="Made with Rust"></a>
    <img src="https://img.shields.io/badge/Platform-Windows-0078D4?logo=windows&logoColor=white" alt="Platform: Windows">
  </p>
  <p><a href="https://github.com/wthrajat/junky/releases/latest"><strong>Download for Windows</strong></a></p>
</div>

## Download

1. Grab **`junky-gui.exe`** from the [latest release](https://github.com/wthrajat/junky/releases/latest) (or `junky.exe` for the CLI).
2. Run it. The build is unsigned, so SmartScreen asks once — choose More info, then Run anyway.
3. Pick categories, press **Scan**, review, then **Clean**.

Portable, no installer. Admin rights unlock system locations; everything else works as a normal user.

## Why Junky

- **Scan first, always.** You see every file and its size before anything is deleted.
- **Targeted, not spray-and-pray.** 23 cleaners hit known junk locations instead of hunting the whole drive by extension.
- **Locked files are skipped, never forced.** No crashes, no half-deleted updates.
- **Small and offline.** Native Rust, ~17 MB app, no runtime to install, no account, no telemetry.

## Safety first

- Aggressive cleaners stay off unless you opt in.
- System32, WinSxS, Installer, Package Cache, the update database, signature catalogs, and the Recovery image are never touched — by blocklist, with tests.
- Symlinks are never followed. Cleaning previews first and confirms before deleting.

## What's inside

<details>
<summary><strong>19 safe cleaners</strong> (on by default)</summary>

| Cleaner | Reclaims |
|---|---|
| user-temp | Your temporary files |
| system-temp | Windows Temp plus service temps |
| update-cache | Downloaded update payloads |
| delivery-optimization | Shared update cache, all known locations |
| recycle-bin | Every drive's recycle bin |
| thumbnail-cache | Explorer thumbnail database |
| icon-cache | Explorer icon database |
| internet-cache | Temporary internet files |
| error-reports | Crash report queues |
| crash-dumps | Minidumps and kernel dumps |
| shader-cache | DirectX, NVIDIA, AMD, and Intel caches |
| office-cache | Office sync leftovers |
| app-caches | Discord, Slack, VS Code, and Teams caches |
| defender-history | Defender scan history and logs |
| windows-logs | Setup, servicing, and IIS logs |
| browser-edge | Edge web cache only |
| browser-chrome | Chrome web cache only |
| browser-firefox | Firefox web cache only |
| browser-brave | Brave web cache only |

</details>

<details>
<summary><strong>4 aggressive cleaners</strong> (opt-in)</summary>

| Cleaner | Reclaims |
|---|---|
| prefetch | Launch prefetch traces |
| windows-old | Prior installs and upgrade scraps |
| extension-sweep | Stray temp files and Thumbs.db, all drives |
| font-cache | Font rasterizer cache |

</details>

<details>
<summary><strong>Never touched</strong></summary>

WinSxS component store (use `DISM /Online /Cleanup-Image /StartComponentCleanup` instead), Windows Installer, Package Cache, the update database, signature catalogs, the Office install cache, the Recovery image, and the Search index.

</details>

## CLI for power users

```console
junky scan --include-aggressive --older-than-hours 24
junky clean --yes
```

## Development

See [docs/development.md](docs/development.md) for setup, testing with a fixture tree, project layout, and the release process.

## License

MIT — see [LICENSE](LICENSE).
