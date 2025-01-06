/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        relative: true,
        files: ["./{sources,includes}/**/*.{tera,html}"],
    },
    theme: {
        extend: {},
    },
    plugins: [],
}