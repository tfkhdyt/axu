# Arch linux eXplicit Updates

[![asciicast](https://asciinema.org/a/614481.svg)](https://asciinema.org/a/614481)

`axu` is a cli tool to check your Arch Linux explicitly installed packages updates. Inspired by [@antfu/taze](https://github.com/antfu/taze), but for Arch Linux

## Getting started

### Pre-requisites

- Arch Linux
- `pacman-contrib`
- `yay`
- `gawk`
- `coreutils`
- Rust (make deps)

### Installation

#### AUR

```bash
yay -S axu-bin
```

### Usage

#### Show all explicit updates

```bash
axu
```

#### Show number of updates

```bash
axu -n
```

#### Show major updates only

```bash
axu major # possible values [major, minor, patch, build]
```
