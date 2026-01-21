import { useState, useEffect } from 'react'

export type OS = 'mac' | 'windows' | 'linux' | 'unknown'
export type Arch = 'arm64' | 'x64' | 'unknown'

export interface DeviceInfo {
    os: OS
    arch: Arch
    name: string
}

export function useDeviceDetector(): DeviceInfo {
    const [device, setDevice] = useState<DeviceInfo>({
        os: 'unknown',
        arch: 'unknown',
        name: 'Unknown Device'
    })

    useEffect(() => {
        const platform = window.navigator.platform.toLowerCase()
        const userAgent = window.navigator.userAgent.toLowerCase()

        let os: OS = 'unknown'
        let arch: Arch = 'unknown'

        // OS Detection
        if (platform.includes('mac')) os = 'mac'
        else if (platform.includes('win')) os = 'windows'
        else if (platform.includes('linux')) os = 'linux'

        // Arch Detection
        if (userAgent.includes('arm') || userAgent.includes('aarch64')) arch = 'arm64'
        else if (userAgent.includes('x64') || userAgent.includes('x86_64') || userAgent.includes('amd64')) arch = 'x64'

        // Guessing Arch for Mac (Silicon detection is tricky via JS but we can try)
        if (os === 'mac' && arch === 'unknown') {
            // Modern Macs are mostly ARM, but we'll default to x64 if we can't tell
            // unless we check hardware concurrency or other hints
            if (navigator.maxTouchPoints > 0) arch = 'arm64' // iPad/iPhone
        }

        const names = {
            mac: 'macOS',
            windows: 'Windows',
            linux: 'Linux',
            unknown: 'Unknown OS'
        }

        setDevice({
            os,
            arch,
            name: names[os]
        })
    }, [])

    return device
}
