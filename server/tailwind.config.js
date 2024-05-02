/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "content/**/*.md",
        "frontend/templates/**/*",
        "frontend/main.ts",
    ],
    theme: {
        extend: {
            container: {
                center: true,
                screens: {
                    sm: "100%",
                    md: "100%",
                    lg: "768px",
                    xl: "880px"
                }
            }
        }
    },
    plugins: [
        require('@tailwindcss/typography'),
        require('@tailwindcss/aspect-ratio'),
    ],
};