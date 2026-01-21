import { Download, BookOpen, ArrowRight, Zap, Box, Layers } from 'lucide-react'
import { Link } from 'react-router-dom'
import TerminalWindow from '../components/TerminalWindow'
import StatsTicker from '../components/StatsTicker'
import { RichTypewriter } from '../components/TypewriterEffect'

export default function Home() {
    return (
        <div className="overflow-hidden bg-surface">
            {/* Hero Section */}
            <section className="container mx-auto px-6 pt-32 pb-24 md:pt-48 md:pb-40 relative">
                {/* Background Decor */}
                <div className="absolute top-0 left-1/2 -translate-x-1/2 w-screen h-full opacity-20 pointer-events-none overflow-hidden select-none">
                    <div className="w-[120%] h-[600px] bg-gradient-to-b from-mutation via-accent to-transparent blur-[120px] rounded-[100%] absolute -top-48 left-1/2 -translate-x-1/2 opacity-30"></div>
                </div>

                <div className="max-w-5xl mx-auto text-center relative z-10 flex flex-col items-center">
                    <div className="inline-flex items-center gap-2 mb-10 px-4 py-1.5 bg-surface/50 backdrop-blur-sm rounded-full border border-primary/10 shadow-sm animate-fade-in-down">
                        <span className="w-2 h-2 rounded-full bg-accent animate-pulse"></span>
                        <span className="font-mono text-xs font-bold tracking-widest opacity-60 uppercase">v0.1.0 Public Beta</span>
                    </div>

                    <h1 className="text-4xl md:text-7xl font-bold tracking-tight leading-[1.1] mb-10 text-primary min-h-[3em] md:min-h-[2.5em] px-4 py-2">
                        <RichTypewriter
                            speed={70}
                            delay={500}
                            segments={[
                                { text: "Flexible ", className: "font-sans" },
                                { text: "Runtimes", className: "font-mono text-mutation italic" },
                                { text: "\nfor ", className: "font-sans block mt-2 md:mt-4" },
                                { text: "Flexible ", className: "font-comic text-green-600" },
                                { text: "Environments", className: "font-display text-accent uppercase tracking-tighter" },
                            ]}
                        />
                    </h1>

                    <p className="font-sans text-xl text-primary/60 mb-12 max-w-3xl mx-auto leading-relaxed">
                        Hybrid is a high-performance polyglot engine that lets you write <span className="text-primary font-bold">orchestration in Hybrid</span>, <span className="text-primary font-bold">data in Python</span>, and <span className="text-primary font-bold">bottlenecks in Rust</span>—in a single file.
                    </p>

                    <div className="flex flex-col sm:flex-row justify-center gap-6 w-full sm:w-auto">
                        <Link to="/download" className="bg-primary text-secondary font-bold text-lg px-8 py-4 rounded-sm border border-primary shadow-neobrutalism hover:translate-x-[2px] hover:translate-y-[2px] hover:shadow-none transition-all flex items-center justify-center gap-3 group">
                            <Download size={20} />
                            Install CLI
                            <ArrowRight size={16} className="opacity-0 -ml-2 group-hover:opacity-100 group-hover:ml-0 transition-all" />
                        </Link>
                        <Link to="/docs" className="bg-surface text-primary font-bold text-lg px-8 py-4 rounded-sm border border-primary/20 hover:border-mutation hover:text-mutation shadow-sm hover:shadow-md transition-all flex items-center justify-center gap-3">
                            <BookOpen size={20} />
                            Read the Docs
                        </Link>
                    </div>
                </div>
            </section>

            {/* Stats Ticker */}
            <StatsTicker />

            {/* Content Panels / Features */}
            <section id="features" className="py-24 bg-surface relative">
                <div className="container mx-auto px-6">
                    <div className="grid md:grid-cols-3 gap-8">
                        {/* Panel 1 */}
                        <div className="bg-surface p-8 border border-primary/10 hover:border-primary hover:shadow-neobrutalism transition-all duration-300 group rounded-xl">
                            <div className="w-12 h-12 bg-primary/5 rounded-lg flex items-center justify-center mb-6 group-hover:bg-accent/10 transition-colors">
                                <Zap className="text-primary group-hover:text-accent transition-colors" size={24} />
                            </div>
                            <h3 className="font-display font-bold text-xl mb-3 text-primary">Zero-Cost Bridging</h3>
                            <p className="font-sans text-primary/60 leading-relaxed">
                                Arguments are serialized to JSON and passed via standard streams to specialized language shims, ensuring near-native performance.
                            </p>
                        </div>

                        {/* Panel 2 */}
                        <div className="bg-surface p-8 border border-primary/10 hover:border-primary hover:shadow-neobrutalism transition-all duration-300 group rounded-xl">
                            <div className="w-12 h-12 bg-primary/5 rounded-lg flex items-center justify-center mb-6 group-hover:bg-mutation/10 transition-colors">
                                <Box className="text-primary group-hover:text-mutation transition-colors" size={24} />
                            </div>
                            <h3 className="font-display font-bold text-xl mb-3 text-primary">Isolated Contexts</h3>
                            <p className="font-sans text-primary/60 leading-relaxed">
                                Rust blocks compile on-demand in temporary workspaces. Python runs in transient subprocesses. Your environment stays clean.
                            </p>
                        </div>

                        {/* Panel 3 */}
                        <div className="bg-surface p-8 border border-primary/10 hover:border-primary hover:shadow-neobrutalism transition-all duration-300 group rounded-xl">
                            <div className="w-12 h-12 bg-primary/5 rounded-lg flex items-center justify-center mb-6 group-hover:bg-green-500/10 transition-colors">
                                <Layers className="text-primary group-hover:text-green-600 transition-colors" size={24} />
                            </div>
                            <h3 className="font-display font-bold text-xl mb-3 text-primary">Unified Toolchain</h3>
                            <p className="font-sans text-primary/60 leading-relaxed">
                                LSP support, syntax highlighting, and formatting out of the box. No complex Makefiles or FFI boilerplate required.
                            </p>
                        </div>
                    </div>
                </div>
            </section>

            {/* Code Showcase */}
            <section className="py-24 bg-surface border-t border-primary/5">
                <div className="container mx-auto px-6">
                    <div className="flex flex-col lg:flex-row items-center gap-16">
                        <div className="lg:w-1/2">
                            <h2 className="text-3xl md:text-5xl font-bold mb-6 tracking-tight text-primary">
                                Stop fighting your <span className="text-mutation">build system</span>.
                            </h2>
                            <p className="text-lg text-primary/60 mb-8 leading-relaxed">
                                Write code in the language that makes sense for the task. Let Hybrid handle the glue code, the compilation, and the data marshalling.
                            </p>

                            <ul className="space-y-4 font-mono text-sm text-primary/80">
                                <li className="flex items-center gap-3">
                                    <div className="w-1.5 h-1.5 bg-accent rounded-full"></div>
                                    write("hybrid_orchestration.hyb")
                                </li>
                                <li className="flex items-center gap-3">
                                    <div className="w-1.5 h-1.5 bg-mutation rounded-full"></div>
                                    import {"{"} rust_module {"}"}
                                </li>
                                <li className="flex items-center gap-3">
                                    <div className="w-1.5 h-1.5 bg-green-500 rounded-full"></div>
                                    await python_analytics.process()
                                </li>
                            </ul>
                        </div>

                        <div className="lg:w-1/2 w-full">
                            <TerminalWindow title="main.hyb" className="shadow-2xl shadow-mutation/20">
                                <span className="text-gray-500">// Hybrid automatically routes logic</span>{'\n'}
                                <span className="text-mutation font-bold">string</span> <span className="text-accent font-bold">const</span> name = <span className="text-green-400">"Hybrid User"</span>;{'\n\n'}

                                <span className="text-blue-400 font-bold">#python</span>{'\n'}
                                <span className="text-mutation font-bold">block</span> get_weather(city) {'{'}{'\n'}
                                {'  '}<span className="text-purple-400">import</span> os, requests{'\n'}
                                {'  '}key = os.getenv(<span className="text-green-400">'API_KEY'</span>){'\n'}
                                {'  '}<span className="text-mutation font-bold">return</span> requests.get(f<span className="text-green-400">"...?q={'{'}city{'}'}"</span>).json(){'\n'}
                                {'}'}{'\n\n'}

                                <span className="text-accent font-bold">speak</span>(<span className="text-green-400">"Starting..."</span>);{'\n'}
                                <span className="text-gray-500">// Native await on foreign function</span>{'\n'}
                                <span className="text-mutation font-bold">let</span> report = <span className="text-accent font-bold">await</span> get_weather(<span className="text-green-400">"Tokyo"</span>);
                            </TerminalWindow>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    )
}
