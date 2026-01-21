import { Download as DownloadIcon, ChevronRight, Apple, Layout as Windows, Terminal as Linux, Monitor, Check } from 'lucide-react'
import { useDeviceDetector } from '../components/DeviceDetector'
import TerminalWindow from '../components/TerminalWindow'

export default function Download() {
    const device = useDeviceDetector()

    const platforms = [
        { id: 'mac', name: 'macOS', icon: <Apple size={40} />, archs: ['ARM64', 'Intel'], color: 'text-accent', border: 'hover:border-accent' },
        { id: 'windows', name: 'Windows', icon: <Windows size={40} />, archs: ['x64'], color: 'text-mutation', border: 'hover:border-mutation' },
        { id: 'linux', name: 'Linux', icon: <Linux size={40} />, archs: ['x64', 'ARM64'], color: 'text-green-500', border: 'hover:border-green-500' },
    ]

    return (
        <div className="min-h-screen bg-surface overflow-hidden">
            {/* Mobile Empty State */}
            <div className="md:hidden flex flex-col items-center justify-center min-h-[80vh] px-6 text-center animate-fade-in-up">
                <div className="w-20 h-20 bg-primary/5 rounded-2xl flex items-center justify-center mb-6 border border-primary/10">
                    <Monitor size={40} className="text-primary/40" />
                </div>
                <h2 className="text-2xl font-bold text-primary mb-3 tracking-tight">Desktop Required</h2>
                <p className="text-lg text-primary/60 max-w-xs mx-auto leading-relaxed">
                    The Hybrid Installer is designed for desktop workstation environments. Please continue setup on your computer.
                </p>
            </div>

            {/* Desktop Content */}
            <div className="hidden md:block">
                {/* Hero Section */}
                <header className="container mx-auto px-6 pt-32 pb-20 text-center">
                    <div className="inline-flex items-center gap-2 mb-8 px-4 py-1.5 bg-surface/50 backdrop-blur-sm rounded-full border border-primary/10 shadow-sm">
                        <span className="w-2 h-2 rounded-full bg-accent animate-pulse"></span>
                        <span className="font-mono text-xs font-bold tracking-widest text-primary/60 uppercase">Cross-Platform Binary</span>
                    </div>

                    <h1 className="text-4xl md:text-6xl font-bold mb-6 text-primary tracking-tight">
                        Get Hybrid
                    </h1>
                    <p className="font-sans text-xl text-primary/60 max-w-2xl mx-auto leading-relaxed">
                        Flexible binaries for every system. Start the <span className="text-mutation font-bold">Mutation</span>.
                    </p>
                </header>

                {/* Platform Grid */}
                <section className="container mx-auto px-6 pb-24">
                    <div className="grid md:grid-cols-3 gap-8">
                        {platforms.map((p) => (
                            <div
                                key={p.id}
                                className={`group relative border bg-surface p-8 rounded-xl transition-all duration-300 ${device.os === p.id
                                    ? 'border-primary shadow-neobrutalism scale-105 z-10'
                                    : 'border-primary/10 hover:border-primary/40 hover:-translate-y-1'
                                    }`}
                            >
                                {device.os === p.id && (
                                    <div className="absolute -top-3 left-1/2 -translate-x-1/2 bg-accent text-white px-3 py-0.5 rounded-full text-xs font-bold tracking-wider shadow-lg flex items-center gap-1">
                                        <Check size={12} strokeWidth={3} />
                                        DETECTED
                                    </div>
                                )}

                                <div className="flex flex-col items-center text-center">
                                    <div className={`w-20 h-20 rounded-2xl bg-primary/5 flex items-center justify-center mb-6 group-hover:bg-primary/10 transition-colors ${p.color}`}>
                                        {p.icon}
                                    </div>
                                    <h2 className="text-2xl font-bold mb-4 text-primary">{p.name}</h2>
                                    <div className="flex gap-2 mb-8">
                                        {p.archs.map(arch => (
                                            <span key={arch} className="font-mono text-xs font-bold text-primary/40 bg-primary/5 px-2 py-1 rounded">
                                                {arch}
                                            </span>
                                        ))}
                                    </div>
                                    <ArchButton os={p.id} />
                                </div>
                            </div>
                        ))}
                    </div>
                </section>

                {/* Quick Install Section */}
                <section className="py-24 bg-surface border-t border-primary/5">
                    <div className="container mx-auto px-6 max-w-4xl">
                        <div className="flex flex-col md:flex-row items-center gap-12">
                            <div className="flex-1">
                                <h2 className="text-3xl font-bold mb-4 text-primary tracking-tight">One Liner</h2>
                                <p className="text-lg text-primary/60 mb-6 leading-relaxed">
                                    For Unix systems, auto-detect and install Hybrid globally in seconds.
                                </p>
                                <div className="flex items-center gap-3 text-sm font-medium text-primary/40">
                                    <span className="w-5 h-5 rounded-full border border-primary/20 flex items-center justify-center font-serif italic text-primary/60">i</span>
                                    Root access may be required for global installation.
                                </div>
                            </div>
                            <div className="w-full md:w-[28rem]">
                                <TerminalWindow title="install.sh">
                                    <span className="text-green-400">$</span> curl -sSL https://devhybrid.org/install.sh | sh
                                </TerminalWindow>
                            </div>
                        </div>
                    </div>
                </section>

                {/* Verification Section */}
                <section className="py-24 container mx-auto px-6 max-w-3xl">
                    <div className="text-center mb-16">
                        <h2 className="text-3xl font-bold mb-2 text-primary tracking-tight">Verify Installation</h2>
                    </div>

                    <div className="grid md:grid-cols-2 gap-8">
                        <div className="p-6 rounded-lg border border-primary/10 bg-surface">
                            <div className="flex items-center gap-3 mb-4">
                                <ChevronRight size={20} className="text-accent" />
                                <h3 className="font-bold text-primary">Version Check</h3>
                            </div>
                            <p className="text-sm text-primary/60 mb-4">Ensure you have the latest binary.</p>
                            <div className="bg-primary/5 p-3 rounded font-mono text-sm text-primary font-bold">
                                hybrid --version
                            </div>
                        </div>

                        <div className="p-6 rounded-lg border border-primary/10 bg-surface">
                            <div className="flex items-center gap-3 mb-4">
                                <ChevronRight size={20} className="text-mutation" />
                                <h3 className="font-bold text-primary">System Doctor</h3>
                            </div>
                            <p className="text-sm text-primary/60 mb-4">Check status of bridged runtimes.</p>
                            <div className="bg-primary/5 p-3 rounded font-mono text-sm text-primary font-bold">
                                hybrid doctor
                            </div>
                        </div>
                    </div>
                </section>
            </div>
        </div>
    )
}

function ArchButton({ os }: { os: string }) {
    const handleDownload = () => {
        const url = os === 'windows' ? '/install.ps1' : '/install.sh'
        const link = document.createElement('a')
        link.href = url
        link.download = os === 'windows' ? 'install.ps1' : 'install.sh'
        document.body.appendChild(link)
        link.click()
        document.body.removeChild(link)
    }

    return (
        <button
            onClick={handleDownload}
            className={`w-full bg-primary text-secondary font-bold text-sm py-3 rounded-sm hover:-translate-y-0.5 hover:shadow-lg transition-all uppercase tracking-wider flex items-center justify-center gap-2`}
        >
            <DownloadIcon size={16} strokeWidth={2.5} />
            Download Installer
        </button>
    )
}
