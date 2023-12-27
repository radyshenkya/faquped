/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{html,js,ts}"],
  theme: {
    fontFamily: {
        'heading': ['Ubuntu'],
        'content': ['Roboto']
    },
    container: {
        center: true,
        padding: {
            DEFAULT: '1rem',
            sm: '2rem',
            lg: '2rem',
            xl: "12rem",
        },
    },
    extend: {},
  },
  plugins: [],
}

