export default {
    darkMode: 'class',
    content: [
        "./index.html",
        "./src/**/*.{js,ts,jsx,tsx}",
    ],
    theme: {
        extend: {
            colors: {
                primary: 'var(--color-primary)', // #000000 / #ffffff
                secondary: 'var(--color-secondary)', // #ffffff / #000000
                accent: '#FF4D00', // Comic orange (kept for brand identity)
                mutation: '#7C3AED', // Mutation purple
                surface: 'var(--bg-surface)', // Background color
            },
            fontFamily: {
                sans: ['Inter', 'system-ui', 'sans-serif'], // Primary font
                mono: ['JetBrains Mono', 'monospace'],     // Code font
                display: ['Inter', 'system-ui', 'sans-serif'], // Replaced Bangers with massive bold Inter
                comic: ['Comic Neue', 'cursive'],          // Kept sparingly for "hybrid" feel
            },
            boxShadow: {
                'neobrutalism': '4px 4px 0px 0px var(--shadow-color)',
                'neobrutalism-sm': '2px 2px 0px 0px var(--shadow-color)',
                'neobrutalism-lg': '8px 8px 0px 0px var(--shadow-color)',
            },
            borderWidth: {
                '3': '3px',
            },
            animation: {
                'float': 'float 6s ease-in-out infinite',
                'ticker': 'ticker 40s linear infinite',
                'blink': 'blink 1s step-end infinite',
            },
            keyframes: {
                float: {
                    '0%, 100%': { transform: 'translateY(0)' },
                    '50%': { transform: 'translateY(-10px)' },
                },
                ticker: {
                    '0%': { transform: 'translateX(0)' },
                    '100%': { transform: 'translateX(-50%)' },
                },
                blink: {
                    '0%, 100%': { opacity: '1' },
                    '50%': { opacity: '0' },
                }
            }
        }
    },
    plugins: [],
}
