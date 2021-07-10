<div align="center">
<h1>Macchina</h1>

Fast, minimal and customizable system information frontend.

Linux • macOS • Windows • NetBSD • OpenWrt • Android

<img src="screenshots/preview.png" alt="Preview" />

<a href="https://crates.io/crates/macchina">
    <img src="https://img.shields.io/crates/v/macchina?label=Version" alt="Version" />
</a>

<a href="https://crates.io/crates/macchina">
    <img src="https://img.shields.io/crates/d/macchina?label=Downloads" alt="Downloads" />
</a>

<a href="https://saythanks.io/to/ba.tahaaziz@gmail.com">
    <img src="https://img.shields.io/badge/Say-Thanks-1EAEDB.svg" alt="Say Thanks" />
</a>
</div>

---

# Table of Contents

- [About](#about)
- [Performance](#perf)
- [Minimalism](#minimal)
- [Features](#features)
- [Configuration](#config)
- [Dependencies](#deps)
- [Installation](#install)
- [Platform Support](#platform-support)
- [Contributors](#contributors)

---

# About Macchina <a name="about"></a>

_Macchina_ lets you view system information, like your kernel version, uptime,
memory usage, processor load and much more. It provides you with convenient
features and extensive customization options but still retains its two main
fundamentals: [performance](#perf) and [minimalism](#minimal).

If you're interested in the library _Macchina_ uses to fetch your system
information, check out
[libmacchina](https://github.com/Macchina-CLI/libmacchina), any issues relating
to _fetching_ should be filed on that repository.

---

# Performance <a name="perf"></a>

_Macchina_ is lightning fast, see for yourself:

- Execution time is measured using
  [hyperfine](https://github.com/sharkdp/hyperfine)

## 🐧 Linux

| Command    | Mean [ms] | Min [ms] | Max [ms] |
| :--------- | --------: | -------: | -------: |
| `macchina` | 3.6 ± 0.5 |      3.0 |      5.4 |

## 👩🏽‍💻 macOS

| Command    | Mean [ms] | Min [ms] | Max [ms] |
| :--------- | --------: | -------: | -------: |
| `macchina` | 5.0 ± 0.2 |      4.7 |      5.9 |

## 🚩 NetBSD

| Command    |  Mean [ms] | Min [ms] | Max [ms] |
| :--------- | ---------: | -------: | -------: |
| `macchina` | 17.7 ± 1.6 |     16.8 |     27.2 |

## 💻 Windows

| Command    | Mean [ms] | Min [ms] | Max [ms] |
| :--------- | --------: | -------: | -------: |
| `macchina` | 9.0 ± 0.4 |      8.2 |     11.7 |

---

# Minimalism <a name="minimal"></a>

_Macchina_ first started as a barebones fetcher, but has evolved to incorporate
the many aspects people love to see in their fetcher. This evolution however did
not discard the minimalistic look that was envisioned for the program.

Here's what you can make it look like using the various supported flags.

```
macchina -c white --no-box --no-ascii --no-separator --no-bar-delimiter --bar
```

![Minimal Output Preview](screenshots/minimal.png)

---

# Features <a name="features"></a>

## Themes

_Macchina_ comes equipped with built-in themes that style their readouts, bars
and separators differently. No extra configuration required.

## Bars

Bars provide a way to visualize data, and each theme styles them differently.
They can be enabled using the `--bar` flag.

<div align="center">
<img src="screenshots/bars.png" alt="Preview of the bar flag" />
</div>

## What it displays

- Host Information
- Product Information
- Kernel Information
- Distribution
- Operating System
- Desktop Environment
- Window Manager
- Package Count
  [[?]](https://github.com/Macchina-CLI/macchina/wiki/Package-Manager-Support)
- Shell
- Terminal
- Processor Information
- Processor Usage
- Local IP Address
- Display Resolution
- Uptime
- Memory Usage
- Battery Information
- Palette

## Meet Doctor

_Macchina_ comes with a very special flag, allowing you analyze which elements
failed to fetch, and why. It can differentiate between errors, and warnings, and
its comprehensive output should help you understand where the issue is coming
from.

```
macchina --doctor
```

![Preview of the doctor flag](screenshots/doctor.png)

---

# Configuration <a name="config"></a>

_Macchina_ can be configured through a dotfile,
[macchina.toml](https://github.com/Macchina-CLI/macchina/blob/main/macchina.toml)
is an example dotfile that you can use and build on top of to make it your own.

- In order for _Macchina_ to read your dotfile, you need to place
  `macchina.toml` in `$XDG_CONFIG_HOME/macchina`

You can also create custom themes in `JSON` format and use them instead of the
built-in themes that we provide.

- In order for _Macchina_ to find your custom themes, which are **JSON** files.
  These files need to placed in `$XDG_DATA_HOME/macchina/themes`, here's an
  example of such a theme:

```json
{
  "name": "Carbon",
  "bar": {
    "Custom": {
      "glyph": "ߋ",
      "symbol_open": "[",
      "symbol_close": "]"
    }
  },
  "color": {
    "Rgb": [231, 198, 100]
  },
  "separator": "⇉",
  "separator_color": {
    "Rgb": [158, 208, 114]
  },
  "spacing": 2,
  "padding": 0,
  "block_title": "┤ Carbon ├",
  "abbreviation": "Classic"
}
```

---

# Dependencies <a name="deps"></a>

These dependencies are not required, but they extend what _Macchina_ can do.

### Linux:

- `wmctrl`
- **Gentoo**: `portage-utils`

### NetBSD:

- `wmctrl`

---

# Installation <a name="install"></a>

### 🦀 Cargo

```
cargo install macchina
```

### 📦 Pkgsrc

```
pkgin install macchina
```

### 🍻 Homebrew

```
brew install macchina
```

### Termux

```
pkg install macchina
```

### Arch Linux

```bash
# Install it using the AUR package that downloads the latest release
git clone https://aur.archlinux.org/macchina.git
cd macchina
makepkg -si

# Or you can use the AUR package that pulls straight from the main branch
git clone https://aur.archlinux.org/macchina-git.git macchina
cd macchina
makepkg -si
```

### NixOS

```bash
nix-env -iA nixos.macchina
```

_Macchina's_
[installation wiki page](https://github.com/grtcdr/macchina/wiki/Installation)
lists the other ways you can install the program. You might also prefer running
the [prebuilt binary](https://github.com/grtcdr/macchina/releases) that
corresponds with your operating system.

---

# Platform Support <a name="platform-support"></a>

| Platform  | Support |
| :-------: | :-----: |
| GNU/Linux |    ✓    |
|  NetBSD   |    ✓    |
|   macOS   |    ✓    |
|  Windows  |   80%   |
|  OpenWrt  |    ✓    |
|  Android  |    ✓    |

# 🌍 Contributors <a name="contributors"></a>

[![Crates.io](https://contrib.rocks/image?repo=grtcdr/macchina)](https://github.com/grtcdr/macchina/graphs/contributors)

_Macchina_, like many other open-source projects, would not be where it is right
now without the help of its contributors, thank you all so much!

- Support for **NetBSD** would not have been possible without the help and
  testing of NetBSD/pkgsrc package maintainer
  [pin](https://pkgsrc.se/bbmaint.php?maint=pin@NetBSD.org)
- Support for **macOS** and **Windows** would not have been possible without the
  help, testing and major contributions of
  [123marvin123](https://github.com/123marvin123)
- Support for **OpenWrt** and **Android** was made possible through the
  contributions of [uttarayan21](https://github.com/uttarayan21)
