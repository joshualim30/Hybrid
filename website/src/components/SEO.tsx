import Logo from '../assets/logo.png'
import { Helmet } from 'react-helmet-async'

interface SEOProps {
    title: string
    description?: string
    image?: string
    url?: string
}

export default function SEO({ title, description, image, url = 'https://devhybrid.org' }: SEOProps) {
    const siteTitle = 'Hybrid | Flexible Code for Flexible Environments'
    const defaultDescription = 'Write once, run anything. Hybrid combines the ease of scripting with the raw power of systems programming.'
    const defaultImage = Logo

    const metaTitle = title === 'Hybrid' ? siteTitle : `${title} | Hybrid`
    const metaDescription = description || defaultDescription
    const metaImage = image || defaultImage

    return (
        <Helmet>
            <title>{metaTitle}</title>
            <meta name="description" content={metaDescription} />

            {/* Open Graph */}
            <meta property="og:title" content={metaTitle} />
            <meta property="og:description" content={metaDescription} />
            <meta property="og:image" content={metaImage} />
            <meta property="og:url" content={url} />

            {/* Twitter */}
            <meta name="twitter:title" content={metaTitle} />
            <meta name="twitter:description" content={metaDescription} />
            <meta name="twitter:image" content={metaImage} />
        </Helmet>
    )
}
