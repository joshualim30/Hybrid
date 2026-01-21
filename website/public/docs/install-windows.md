# Windows Installation

Install Hybrid on Windows 10/11 via the Windows Installer (MSI) or Scoop.

## Windows Installer (MSI)

1. Download the latest `hybrid-installer-x64.msi` from the [Releases](https://github.com/joshualim30/hybrid/releases) page.
2. Double-click the file to run the installer wizard.
3. Follow the prompts to install Hybrid. The installer will automatically add `hybrid` to your user PATH.
4. Open PowerShell or Command Prompt and run:
   ```powershell
   hybrid --version
   ```

## Scoop (Recommended for Devs)

If you use Scoop:

```powershell
scoop bucket add hybrid https://github.com/hybrid-lang/scoop
scoop install hybrid
```

## System Requirements

- Windows 10 Version 2004 or later
- [Microsoft Visual C++ Redistributable](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist) may be required for some Rust extensions.
