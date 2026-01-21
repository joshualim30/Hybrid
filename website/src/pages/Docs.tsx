import { Terminal, Code2, Rocket, Shield, Cpu, Command, AppWindow } from 'lucide-react'
import { useState, useEffect } from 'react'
import TerminalWindow from '../components/TerminalWindow'
import SEO from '../components/SEO'

export default function Docs() {
    const [activeSection, setActiveSection] = useState('getting-started')
    const [content, setContent] = useState('')
    const [loading, setLoading] = useState(true)

    const groups = [
        {
            title: "Start Here",
            items: [
                { id: 'getting-started', title: 'Introduction', icon: <Rocket size={16} /> },
                { id: 'install-macos', title: 'macOS', icon: <Command size={16} /> },
                { id: 'install-windows', title: 'Windows', icon: <AppWindow size={16} /> },
                { id: 'install-linux', title: 'Linux', icon: <Terminal size={16} /> },
            ]
        },
        {
            title: "Core Concepts",
            items: [
                { id: 'cli-interface', title: 'CLI Usage', icon: <Terminal size={16} /> },
                { id: 'mutation-syntax', title: 'Mutation Syntax', icon: <Code2 size={16} /> },
                { id: 'type-marshaling', title: 'Type Marshaling', icon: <Cpu size={16} /> },
                { id: 'security', title: 'Security', icon: <Shield size={16} /> },
            ]
        }
    ]

    useEffect(() => {
        setLoading(true)
        fetch(`/docs/${activeSection}.md`)
            .then(res => res.text())
            .then(text => {
                setContent(text)
                setLoading(false)
            })
            .catch(() => {
                setContent("# Error\nCould not load documentation.")
                setLoading(false)
            })
    }, [activeSection])

    return (
        <div className="min-h-screen bg-surface pt-32 pb-24">
            <SEO
                title="Documentation"
                description="Learn how to install, configure, and code with Hybrid. Comprehensive guides and API references."
            />
            <div className="container mx-auto px-6 grid md:grid-cols-[260px_1fr] gap-16">
                {/* Sidebar */}
                <aside className="hidden md:block">
                    <nav className="sticky top-32 space-y-8">
                        {groups.map((group, i) => (
                            <div key={i}>
                                <div className="font-mono text-xs font-bold text-primary/40 uppercase tracking-widest mb-4 px-4">
                                    {group.title}
                                </div>
                                <div className="space-y-1">
                                    {group.items.map(s => (
                                        <button
                                            key={s.id}
                                            onClick={() => setActiveSection(s.id)}
                                            className={`w-full flex items-center gap-3 px-4 py-2.5 rounded-sm font-sans text-sm font-medium transition-all ${activeSection === s.id
                                                ? 'bg-primary/5 text-primary border-l-2 border-accent'
                                                : 'text-primary/60 hover:text-primary hover:bg-primary/5'
                                                }`}
                                        >
                                            <span className={activeSection === s.id ? 'text-accent' : 'opacity-70'}>{s.icon}</span>
                                            {s.title}
                                        </button>
                                    ))}
                                </div>
                            </div>
                        ))}
                    </nav>
                </aside>

                {/* Content */}
                <main className="max-w-4xl min-h-[500px]">
                    <header className="mb-10 border-b border-primary/10 pb-6 flex items-center justify-between">
                        <div className="inline-flex items-center gap-2">
                            <span className="w-2 h-2 rounded-full bg-mutation animate-pulse"></span>
                            <span className="font-mono text-xs font-bold text-primary/60 tracking-widest uppercase">
                                Docs // v0.1.0 Alpha
                            </span>
                        </div>
                        {loading && <div className="text-sm font-mono text-accent animate-pulse">Loading...</div>}
                    </header>

                    <div className="animate-fade-in-up">
                        <MarkdownViewer content={content} />
                    </div>
                </main>
            </div>
        </div>
    )
}

function MarkdownViewer({ content }: { content: string }) {
    // Extremely basic Markdown Parser for the specific needs of this site.
    // Splits by lines and rough regex.
    const parts = content.split(/(\n```[\s\S]*?```)/g);

    return (
        <div className="space-y-6">
            {parts.map((part, index) => {
                if (part.startsWith('\n```')) {
                    // Code block
                    const clean = part.replace(/^\n```\w*\n?/, '').replace(/```$/, '');
                    const title = part.match(/^\n```(\w*)/)?.[1] || 'Code'
                    return <TerminalWindow key={index} title={title}>{clean}</TerminalWindow>
                } else {
                    // Text block - process headers and tables roughly
                    return <div key={index} dangerouslySetInnerHTML={{ __html: parseMarkdownText(part) }} className="markdown-body" />
                }
            })}
        </div>
    )
}

function parseMarkdownText(text: string) {
    let ht = text
        .replace(/^# (.*$)/gim, '<h1 class="text-4xl font-bold mb-6 text-primary tracking-tight">$1</h1>')
        .replace(/^## (.*$)/gim, '<h2 class="text-2xl font-bold mt-8 mb-4 text-primary border-l-4 border-accent pl-4">$1</h2>')
        .replace(/^### (.*$)/gim, '<h3 class="text-xl font-bold mt-6 mb-3 text-primary">$1</h3>')
        .replace(/\*\*(.*)\*\*/gim, '<strong>$1</strong>')
        .replace(/`([^`]+)`/gim, '<code class="bg-primary/10 px-1.5 py-0.5 rounded font-mono text-sm text-mutation font-bold">$1</code>')
        .replace(/\n\n/gim, '<br/><br/>')

    // Simple table parser
    if (ht.includes('|')) {
        const rows = ht.trim().split('\n').filter(r => r.includes('|'));
        if (rows.length > 2) {
            // It's a table
            const header = rows[0].split('|').filter(c => c.trim()).map(c => `<th class="p-3 text-left border-b border-primary/20 bg-primary/5">${c.trim()}</th>`).join('')
            const body = rows.slice(2).map(r => `<tr>${r.split('|').filter(c => c.trim()).map(c => `<td class="p-3 border-b border-primary/10 font-mono text-sm">${c.trim()}</td>`).join('')}</tr>`).join('')
            return `<div class="overflow-x-auto my-6 border border-primary/10 rounded-lg"><table class="w-full text-left border-collapse"><thead><tr>${header}</tr></thead><tbody>${body}</tbody></table></div>`
        }
    }

    return ht;
}
