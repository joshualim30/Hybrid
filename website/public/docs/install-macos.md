# macOS Installation

Install Hybrid on macOS using our official Homebrew tap or by downloading the universal binary.

## Homebrew (Recommended)

The easiest way to install Hybrid is via Homebrew. This will manage updates automatically.

```bash
brew tap hybrid-lang/tap
brew install hybrid
```

## Manual Installation (Universal Binary)

1. Download the latest `hybrid-macos-universal.tar.gz` from the [Releases](https://github.com/joshualim30/hybrid/releases) page.
2. Extract the archive:
   ```bash
   tar -xvf hybrid-macos-universal.tar.gz
   ```
3. Move the binary to your path:
   ```bash
   sudo mv hybrid /usr/local/bin/
   ```
4. Verify installation:
   ```bash
   hybrid --version
   ```

## Apple Silicon vs Intel

Our universal binary supports both M1/M2/M3 chips and Intel processors natively. No strictly required Rosetta translation, though Python bridges may rely on your system Python architecture.
