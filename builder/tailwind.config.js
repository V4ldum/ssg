/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        relative: true,
        files: ["./{source,includes}/**/*.{tera,html}"],
    },
    theme: {
        extend: {},
    },
    plugins: [],
}