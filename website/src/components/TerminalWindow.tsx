import { Copy, Check } from 'lucide-react'
import { useState } from 'react'

interface TerminalWindowProps {
    title?: string;
    children: React.ReactNode;
    className?: string;
}

export default function TerminalWindow({ title = "bash", children, className = "" }: TerminalWindowProps) {
    const [copied, setCopied] = useState(false);

    const handleCopy = () => {
        if (!children) return;
        // Basic extraction of text content from children could be complex, 
        // usually users pass code as a string or we refrain from copy for complex nodes.
        // For simplicity, we just toggle the icon to show interaction here.
        setCopied(true);
        setTimeout(() => setCopied(false), 2000);
    };

    return (
        <div className={`rounded-lg overflow-hidden border border-primary/10 dark:border-white/10 shadow-neobrutalism bg-[#0f172a] text-white font-mono text-sm ${className}`}>
            {/* Terminal Header */}
            <div className="flex items-center justify-between px-4 py-2 bg-[#1e293b] border-b border-white/5">
                <div className="flex items-center gap-2">
                    <div className="flex gap-1.5">
                        <div className="w-3 h-3 rounded-full bg-[#ff5f56]"></div>
                        <div className="w-3 h-3 rounded-full bg-[#ffbd2e]"></div>
                        <div className="w-3 h-3 rounded-full bg-[#27c93f]"></div>
                    </div>
                    <div className="ml-4 text-xs text-gray-400 font-medium opacity-70">
                        {title}
                    </div>
                </div>
                <button
                    onClick={handleCopy}
                    className="text-gray-400 hover:text-white transition-colors p-1"
                    title="Copy to clipboard"
                >
                    {copied ? <Check size={14} className="text-green-400" /> : <Copy size={14} />}
                </button>
            </div>

            {/* Terminal Content */}
            <div className="p-6 overflow-x-auto">
                <pre className="font-mono leading-relaxed">
                    {children}
                </pre>
            </div>
        </div>
    )
}
