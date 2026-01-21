import { useState, useEffect } from 'react'
import { Github, Menu, X } from 'lucide-react'
import { Link, Outlet, useLocation } from 'react-router-dom'

export default function Layout() {
    const [isMenuOpen, setIsMenuOpen] = useState(false)
    const location = useLocation()
    const [scrolled, setScrolled] = useState(false)

    useEffect(() => {
        const handleScroll = () => {
            setScrolled(window.scrollY > 20)
        }
        window.addEventListener('scroll', handleScroll)
        return () => window.removeEventListener('scroll', handleScroll)
    }, [])

    useEffect(() => {
        const path = location.pathname.substring(1)
        const pageName = path.charAt(0).toUpperCase() + path.slice(1)
        // More professional code-like title
        document.title = pageName ? `Hybrid :: ${pageName}` : 'Hybrid :: Flexible Runtime'
        window.scrollTo(0, 0)
    }, [location])

    // System Dark Mode Handler
    useEffect(() => {
        const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')

        const applyTheme = (isDark: boolean) => {
            if (isDark) {
                document.documentElement.classList.add('dark')
            } else {
                document.documentElement.classList.remove('dark')
            }
        }

        // Apply initially
        applyTheme(mediaQuery.matches)

        // Listen for changes
        const handler = (e: MediaQueryListEvent) => applyTheme(e.matches)
        mediaQuery.addEventListener('change', handler)
        return () => mediaQuery.removeEventListener('change', handler)
    }, [])

    return (
        <div className="min-h-screen flex flex-col font-sans bg-surface text-primary selection:bg-mutation selection:text-white transition-colors duration-200">
            {/* Sticky Navbar with Scroll State */}
            <nav className={`fixed w-full z-50 transition-all duration-300 ${scrolled
                ? 'bg-surface/90 backdrop-blur-md border-b border-primary/10 py-3 shadow-sm'
                : 'bg-transparent border-b border-transparent py-5'
                }`}>
                <div className="container mx-auto px-6 flex justify-between items-center">
                    <Link to="/" className="text-xl font-bold tracking-tight flex items-center gap-2 group">
                        <div className="bg-primary text-secondary w-8 h-8 flex items-center justify-center rounded-sm shadow-neobrutalism-sm group-hover:shadow-none group-hover:translate-x-[2px] group-hover:translate-y-[2px] transition-all">
                            <span className="font-display text-lg transform -rotate-2">H</span>
                        </div>
                        <span className="font-display uppercase">Hybrid</span>
                    </Link>

                    <div className="hidden md:flex gap-8 items-center font-mono text-sm font-medium tracking-tight">
                        <Link to="/languages" className="hover:text-mutation hover:underline decoration-2 underline-offset-4 transition-all">Languages</Link>
                        <Link to="/docs" className="hover:text-accent hover:underline decoration-2 underline-offset-4 transition-all">Docs</Link>
                        <Link to="/download" className="hover:text-mutation hover:underline decoration-2 underline-offset-4 transition-all">Install</Link>

                        <div className="h-4 w-[2px] bg-primary/20"></div>

                        <a
                            href="https://github.com/joshualim30/hybrid"
                            target="_blank"
                            rel="noreferrer"
                            className="bg-primary text-secondary px-4 py-1.5 font-bold text-xs uppercase tracking-wider border border-primary shadow-neobrutalism-sm hover:shadow-none hover:translate-x-[2px] hover:translate-y-[2px] transition-all flex items-center gap-2"
                        >
                            <Github size={14} />
                            GitHub
                        </a>
                    </div>

                    <button onClick={() => setIsMenuOpen(!isMenuOpen)} className="md:hidden p-2">
                        {isMenuOpen ? <X size={24} /> : <Menu size={24} />}
                    </button>
                </div>

                {/* Mobile Menu */}
                {isMenuOpen && (
                    <div className="md:hidden absolute top-full left-0 w-full bg-surface border-b border-primary/10 p-6 flex flex-col gap-4 shadow-xl animate-fade-in-down">
                        <Link to="/languages" className="text-lg font-mono font-bold hover:text-mutation" onClick={() => setIsMenuOpen(false)}>Languages</Link>
                        <Link to="/docs" className="text-lg font-mono font-bold hover:text-accent" onClick={() => setIsMenuOpen(false)}>Docs</Link>
                        <Link to="/download" className="text-lg font-mono font-bold hover:text-mutation" onClick={() => setIsMenuOpen(false)}>Install CLI</Link>
                        <hr className="border-primary/10" />
                        <a href="https://github.com/joshualim30/hybrid" target="_blank" rel="noreferrer" className="flex items-center gap-2 text-lg font-mono font-bold hover:text-primary/70">
                            <Github size={20} />
                            GitHub
                        </a>
                    </div>
                )}
            </nav>

            {/* Main Content */}
            <main className="flex-grow">
                <Outlet />
            </main>

            {/* Footer */}
            <footer className="border-t border-primary/10 py-12 bg-surface text-primary relative z-10">
                <div className="container mx-auto px-6">
                    <div className="grid md:grid-cols-4 gap-8 mb-8">
                        <div className="col-span-1 md:col-span-2">
                            <Link to="/" className="flex items-center gap-3 mb-4 group w-fit">
                                <div className="w-8 h-8 bg-primary text-secondary flex items-center justify-center font-display text-lg rounded-sm shadow-neobrutalism-sm group-hover:shadow-none transition-all">
                                    H
                                </div>
                                <span className="font-display text-lg uppercase tracking-wider">Hybrid</span>
                            </Link>
                            <p className="font-mono text-xs opacity-60 max-w-xs leading-relaxed">
                                A high-performance polyglot runtime engine. Write the orchestration in Hybrid, the data in Python, and the bottlenecks in Rust.
                            </p>
                        </div>

                        <div>
                            <h4 className="font-mono text-xs font-bold uppercase tracking-wider opacity-50 mb-4">Resources</h4>
                            <div className="flex flex-col gap-2 font-mono text-sm">
                                <Link to="/docs" className="hover:text-accent transition-colors">Documentation</Link>
                                <Link to="/languages" className="hover:text-mutation transition-colors">Language Support</Link>
                                <Link to="/download" className="hover:text-accent transition-colors">Installation</Link>
                            </div>
                        </div>

                        <div>
                            <h4 className="font-mono text-xs font-bold uppercase tracking-wider opacity-50 mb-4">Community</h4>
                            <div className="flex flex-col gap-2 font-mono text-sm">
                                <a href="https://github.com/joshualim30/hybrid" target="_blank" rel="noreferrer" className="hover:text-mutation transition-colors">GitHub</a>
                                <a href="https://discord.gg/faAehkDX3C" target="_blank" rel="noreferrer" className="hover:text-accent transition-colors">Discord</a>
                                <a href="https://twitter.com/joshualim30" target="_blank" rel="noreferrer" className="hover:text-mutation transition-colors">Twitter</a>
                            </div>
                        </div>
                    </div>

                    <div className="pt-8 border-t border-primary/5 flex flex-col md:flex-row justify-between items-center gap-4 text-[10px] font-mono opacity-40 uppercase tracking-widest">
                        <span>© 2026 Joshua Lim. MIT License.</span>
                        <p className="flex items-center gap-2 hover:text-mutation transition-colors">
                            Made with ❤️ thanks to the open source community :&#41;
                        </p>
                    </div>
                </div>
            </footer>
        </div>
    )
}
