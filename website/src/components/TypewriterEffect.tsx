import { useState, useEffect } from 'react'

/**
 * A custom hook that returns a typed version of the input text
 * @param text The text to type out
 * @param speed Typing speed in ms (default 50)
 * @param delay Delay before starting in ms (default 1000)
 * @returns The currently typed text
 */
export function useTypewriter(text: string, speed = 50, delay = 1000) {
    const [displayText, setDisplayText] = useState('')

    useEffect(() => {
        let i = 0
        setDisplayText('')

        const timeout = setTimeout(() => {
            const interval = setInterval(() => {
                setDisplayText(text.substring(0, i + 1))
                i++
                if (i === text.length) clearInterval(interval)
            }, speed)
            return () => clearInterval(interval)
        }, delay)

        return () => clearTimeout(timeout)
    }, [text, speed, delay])

    return displayText
}

interface RichSegment {
    text: string;
    className?: string;
}

export function RichTypewriter({ segments, speed = 50, delay = 1000 }: { segments: RichSegment[], speed?: number, delay?: number }) {
    const [displayedSegments, setDisplayedSegments] = useState<RichSegment[]>([])
    const [currentSegmentIndex, setCurrentSegmentIndex] = useState(0)
    const [currentCharIndex, setCurrentCharIndex] = useState(0)
    const [started, setStarted] = useState(false)

    useEffect(() => {
        const timer = setTimeout(() => setStarted(true), delay)
        return () => clearTimeout(timer)
    }, [delay])

    useEffect(() => {
        if (!started) return
        if (currentSegmentIndex >= segments.length) return

        const timeout = setTimeout(() => {
            const currentSegment = segments[currentSegmentIndex]

            if (currentCharIndex < currentSegment.text.length) {
                // Typing current segment
                const updatedSegments = [...displayedSegments]
                if (updatedSegments[currentSegmentIndex]) {
                    updatedSegments[currentSegmentIndex] = {
                        ...currentSegment,
                        text: currentSegment.text.substring(0, currentCharIndex + 1)
                    }
                } else {
                    updatedSegments[currentSegmentIndex] = {
                        ...currentSegment,
                        text: currentSegment.text.substring(0, 1)
                    }
                }
                setDisplayedSegments(updatedSegments)
                setCurrentCharIndex(prev => prev + 1)
            } else {
                // Move to next segment
                setCurrentSegmentIndex(prev => prev + 1)
                setCurrentCharIndex(0)
            }
        }, speed)

        return () => clearTimeout(timeout)
    }, [started, currentSegmentIndex, currentCharIndex, segments, speed, displayedSegments])

    return (
        <span>
            {displayedSegments.map((seg, i) => (
                <span key={i} className={seg.className}>{seg.text}</span>
            ))}
            {currentSegmentIndex < segments.length && <BlinkingCursor />}
        </span>
    )
}

export function BlinkingCursor() {
    return (
        <span className="inline-block w-[3px] h-[1em] bg-accent ml-1 align-middle animate-blink shadow-[0_0_8px_rgba(255,77,0,0.5)]"></span>
    )
}
