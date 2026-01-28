import LanguageTicker from '../components/LanguageTicker'
import { CheckCircle2, Circle, ArrowRight } from 'lucide-react'

import SEO from '../components/SEO'

export default function Languages() {
    return (
        <div className="min-h-screen bg-surface overflow-hidden">
            <SEO
                title="Roadmap"
                description="Explore the future of Hybrid: Foreign Function Interfaces (FFI) for Python, Node.js, and more."
            />
            {/* Hero Section */}
            <section className="container mx-auto px-6 pt-32 pb-20 text-center">
                <div className="inline-flex items-center gap-2 mb-8 px-4 py-1.5 bg-surface/50 backdrop-blur-sm rounded-full border border-primary/10 shadow-sm">
                    <span className="w-2 h-2 rounded-full bg-mutation animate-pulse"></span>
                    <span className="font-mono text-xs font-bold tracking-widest text-primary/60 uppercase">Ecosystem Ready</span>
                </div>

                <h1 className="text-4xl md:text-6xl font-bold mb-6 text-primary tracking-tight">
                    Ecosystem Support
                </h1>
                <p className="font-sans text-xl text-primary/60 max-w-2xl mx-auto leading-relaxed">
                    Flexible tools for every developer. Hybrid integrates with your favorite runtimes at the <span className="font-bold text-primary">memory level</span>.
                </p>
            </section>

            <div className="pb-16">
                <LanguageTicker />
            </div>

            {/* Matrix Section */}
            <section className="py-24 bg-surface border-t border-primary/5">
                <div className="container mx-auto px-6">
                    <div className="grid md:grid-cols-3 gap-8">
                        {[
                            { name: 'PYTHON', status: 'STABLE', interface: 'C-API/PyO3', efficiency: 'ZERO-COPY', color: 'text-[#3776AB]', borderColor: 'border-[#3776AB]' },
                            { name: 'RUST', status: 'STABLE', interface: 'NATIVE FFI', efficiency: 'MAX SPEED', color: 'text-accent', borderColor: 'border-accent' },
                            { name: 'NODE.JS', status: 'BETA', interface: 'V8 BRIDGE', efficiency: 'BALANCED', color: 'text-[#339933]', borderColor: 'border-[#339933]' }
                        ].map((runtime, i) => (
                            <div key={i} className="bg-surface border border-primary/10 p-8 rounded-xl hover:border-primary hover:shadow-neobrutalism transition-all duration-300 group">
                                <h3 className={`font-display font-bold text-3xl mb-6 ${runtime.color}`}>{runtime.name}</h3>
                                <div className="space-y-4 font-mono font-medium text-sm text-primary/80">
                                    <div className="flex justify-between items-center border-b border-primary/5 pb-2">
                                        <span className="opacity-40 uppercase tracking-wider">Status</span>
                                        <div className="flex items-center gap-2">
                                            <div className={`w-1.5 h-1.5 rounded-full ${runtime.status === 'STABLE' ? 'bg-green-500' : 'bg-yellow-500'}`}></div>
                                            <span>{runtime.status}</span>
                                        </div>
                                    </div>
                                    <div className="flex justify-between items-center border-b border-primary/5 pb-2">
                                        <span className="opacity-40 uppercase tracking-wider">Interface</span>
                                        <span>{runtime.interface}</span>
                                    </div>
                                    <div className="flex justify-between items-center pb-2">
                                        <span className="opacity-40 uppercase tracking-wider">Efficiency</span>
                                        <span>{runtime.efficiency}</span>
                                    </div>
                                </div>
                            </div>
                        ))}
                    </div>
                </div>
            </section>

            {/* Roadmap Panels */}
            <section className="container mx-auto px-6 py-24 border-t border-primary/5">
                <div className="relative">
                    <div className="absolute left-1/2 -top-6 -translate-x-1/2 bg-surface px-4 font-mono text-xs font-bold text-primary/40 uppercase tracking-widest">
                        Roadmap
                    </div>
                    <div className="grid lg:grid-cols-2 gap-12 pt-12">
                        {/* Phase 1 */}
                        <div className="relative group isolate">
                            <div className="absolute -left-6 -top-10 font-display text-9xl text-primary/5 select-none -z-10 transition-transform group-hover:translate-x-2">01</div>
                            <div className="relative z-10 pl-4 pt-8">
                                <h2 className="text-2xl font-bold mb-6 text-primary tracking-tight uppercase">The Core</h2>
                                <ul className="space-y-4">
                                    <li className="flex items-start gap-3 text-primary/70">
                                        <CheckCircle2 size={20} className="text-green-500 mt-1 shrink-0" />
                                        <span>Universal Type Marshaling for primitive and complex types</span>
                                    </li>
                                    <li className="flex items-start gap-3 text-primary/70">
                                        <CheckCircle2 size={20} className="text-green-500 mt-1 shrink-0" />
                                        <span>Python 3.10+ Integration with virtual environment support</span>
                                    </li>
                                    <li className="flex items-start gap-3 text-primary/70">
                                        <CheckCircle2 size={20} className="text-green-500 mt-1 shrink-0" />
                                        <span>Rust Crates Support via inline Cargo.toml definitions</span>
                                    </li>
                                </ul>
                            </div>
                        </div>

                        {/* Phase 2 */}
                        <div className="relative group isolate">
                            <div className="absolute -left-6 -top-10 font-display text-9xl text-primary/5 select-none -z-10 transition-transform group-hover:translate-x-2">02</div>
                            <div className="relative z-10 pl-4 pt-8">
                                <h2 className="text-2xl font-bold mb-6 text-primary tracking-tight uppercase">The Edge</h2>
                                <ul className="space-y-4">
                                    <li className="flex items-start gap-3 text-primary/70">
                                        <Circle size={20} className="text-primary/30 mt-1 shrink-0" />
                                        <span>WASM Execution Targets for browser-based orchestration</span>
                                    </li>
                                    <li className="flex items-start gap-3 text-primary/70">
                                        <Circle size={20} className="text-primary/30 mt-1 shrink-0" />
                                        <span>Node.js V8 Bridge for server-side JavaScript integration</span>
                                    </li>
                                    <li className="flex items-start gap-3 text-primary/70">
                                        <Circle size={20} className="text-primary/30 mt-1 shrink-0" />
                                        <span>Distributed Memory Sync across network boundaries</span>
                                    </li>
                                </ul>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            {/* Bottom CTA */}
            <section className="py-24 container mx-auto px-6 text-center">
                <div className="max-w-3xl mx-auto border border-primary/10 rounded-xl p-10 bg-surface shadow-lg hover:shadow-xl transition-all">
                    <h2 className="text-2xl font-bold mb-4 text-primary">Missing your runtime?</h2>
                    <p className="text-lg text-primary/60 mb-8 max-w-xl mx-auto">
                        Help us expand the Hybrid ecosystem. Contribute a bridge or request a new integration on GitHub.
                    </p>
                    <a
                        href="https://github.com/Creating-Real/hybrid/issues"
                        target="_blank"
                        rel="noreferrer"
                        className="inline-flex items-center gap-2 bg-primary text-secondary font-bold text-base px-8 py-3 rounded-sm border border-primary shadow-neobrutalism hover:translate-x-[2px] hover:translate-y-[2px] hover:shadow-none transition-all uppercase tracking-wider"
                    >
                        Request Language
                        <ArrowRight size={18} />
                    </a>
                </div>
            </section>
        </div>
    )
}
