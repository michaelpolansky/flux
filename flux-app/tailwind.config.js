/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.rs",
  ],
  theme: {
    extend: {
      colors: {
        background: "#09090b", // zinc-950
        foreground: "#fafafa", // zinc-50
        primary: {
          DEFAULT: "#dc2626", // red-600 (Active Trigs)
          foreground: "#fafafa",
        },
        muted: {
          DEFAULT: "#27272a", // zinc-800
          foreground: "#a1a1aa", // zinc-400
        },
        accent: {
          DEFAULT: "#27272a", // zinc-800
          foreground: "#fafafa",
        },
        card: {
          DEFAULT: "#09090b",
          foreground: "#fafafa",
        },
        border: "#27272a", // zinc-800
        input: "#27272a",
      },
      fontFamily: {
        sans: ['Inter', 'sans-serif'],
      },
    },
  },
  plugins: [],
}
