import { Zap, Shield, Cpu, Box, Layers, Globe } from 'lucide-react'

export default function StatsTicker() {
    const stats = [
        { label: "ZERO-COPY BRIDGING", icon: <Zap size={16} /> },
        { label: "100% TYPE SAFETY", icon: <Shield size={16} /> },
        { label: "SUB-MILLISECOND LATENCY", icon: <Cpu size={16} /> },
        { label: "ISOLATED CONTEXTS", icon: <Box size={16} /> },
        { label: "CROSS-PLATFORM BINARIES", icon: <Globe size={16} /> },
        { label: "UNIFIED TOOLCHAIN", icon: <Layers size={16} /> },
    ]

    return (
        <div className="py-8 overflow-hidden bg-surface border-y border-primary/5 relative">
            {/* Fade masks */}
            <div className="absolute left-0 top-0 bottom-0 w-32 bg-gradient-to-r from-surface to-transparent z-10 pointer-events-none"></div>
            <div className="absolute right-0 top-0 bottom-0 w-32 bg-gradient-to-l from-surface to-transparent z-10 pointer-events-none"></div>

            <div className="relative w-full flex overflow-x-hidden group">
                {/* 
                   We used to have 3 sets. If we use 2 sets and -50% translation, 
                   it works perfectly if the track width is defined by the content.
                   tailwind 'animate-ticker' does transform: translateX(-50%).
                */}
                <div className="animate-ticker flex whitespace-nowrap group-hover:[animation-play-state:paused] items-center">
                    {[...stats, ...stats, ...stats, ...stats].map((stat, i) => (
                        <div key={i} className="flex items-center gap-3 mx-8 opacity-60 hover:opacity-100 transition-opacity duration-300">
                            <div className="text-accent">{stat.icon}</div>
                            <span className="font-mono font-bold text-sm tracking-widest text-primary">{stat.label}</span>
                            <div className="w-1.5 h-1.5 bg-primary/20 rounded-full ml-8"></div>
                        </div>
                    ))}
                </div>
            </div>
        </div>
    )
}
