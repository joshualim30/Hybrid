# Linux Installation

Hybrid supports most major Linux distributions (Ubuntu, Debian, Fedora, Arch).

## Automated Script

The quickest way to install is via our shell script:

```bash
curl -fsSL https://get.hybrid-lang.org/install.sh | sh
```

This script detects your architecture and distribution, downloads the correct binary, and installs it to `~/.hybrid/bin`.

## DEB Package (Debian/Ubuntu)

```bash
wget https://github.com/joshualim30/hybrid/releases/latest/download/hybrid_amd64.deb
sudo dpkg -i hybrid_amd64.deb
```

## RPM Package (Fedora/RHEL)

```bash
sudo rpm -i https://github.com/joshualim30/hybrid/releases/latest/download/hybrid.x86_64.rpm
```

## Arch Linux (AUR)

```bash
yay -S hybrid-bin
```
