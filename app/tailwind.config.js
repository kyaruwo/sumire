/*

tailwindcss v3.4.0

cd app & tailwindcss\tailwindcss.exe -i tailwindcss\tailwind.css -o css\sumire.css --minify --watch

*/

/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["index.html", "html/*.html", "js/*.js"],
    theme: {
        extend: {},
    },
    plugins: [],
};
