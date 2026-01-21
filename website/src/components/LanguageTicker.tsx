export default function LanguageTicker() {
    const targets = [
        { name: "PYTHON", color: "#3776AB", label: "v3.10+" },
        { name: "RUST", color: "#686868", label: "Stable" },
        { name: "NODE", color: "#339933", label: "v18+" },
        { name: "GO", color: "#00ADD8", label: "v1.20+" },
        { name: "C++", color: "#00599C", label: "C++20" },
        { name: "SWIFT", color: "#F05138", label: "5.7+" },
        { name: "WASM", color: "#654FF0", label: "Standard" },
    ]

    return (
        <div className="py-12 overflow-hidden bg-surface border-y border-primary/5 dark:border-white/5 relative">

            {/* Fade masks */}
            <div className="absolute left-0 top-0 bottom-0 w-32 bg-gradient-to-r from-surface to-transparent z-10 pointer-events-none"></div>
            <div className="absolute right-0 top-0 bottom-0 w-32 bg-gradient-to-l from-surface to-transparent z-10 pointer-events-none"></div>

            <div className="relative w-full flex overflow-x-hidden group">
                <div className="animate-ticker flex whitespace-nowrap group-hover:[animation-play-state:paused]">
                    {/* Triple duplication for extra smoothness on wide screens */}
                    {[...targets, ...targets, ...targets].map((target, i) => (
                        <div key={i} className="flex items-center gap-4 mx-8 opacity-50 grayscale hover:grayscale-0 hover:opacity-100 transition-all duration-300 cursor-default">
                            <div className="w-2 h-2 rounded-sm" style={{ backgroundColor: target.color }}></div>
                            <div className="flex flex-col">
                                <span className="font-display font-bold text-xl tracking-tight leading-none" style={{ color: target.color }}>{target.name}</span>
                                <span className="font-mono text-[10px] opacity-60 leading-none">{target.label}</span>
                            </div>
                        </div>
                    ))}
                </div>
            </div>
        </div>
    )
}
