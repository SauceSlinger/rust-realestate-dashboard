/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  darkMode: 'class', // Enable class-based dark mode
  theme: {
    extend: {
      colors: {
        primary: {
          50: '#f0f9ff',
          100: '#e0f2fe',
          200: '#bae6fd',
          300: '#7dd3fc',
          400: '#38bdf8',
          500: '#0ea5e9',
          600: '#0284c7',
          700: '#0369a1',
          800: '#075985',
          900: '#0c4a6e',
          950: '#082f49',
        },
      },
      backgroundColor: {
        'light': '#ffffff',
        'light-secondary': '#f9fafb',
        'dark': '#0f172a',
        'dark-secondary': '#1e293b',
        'dark-card': '#1e293b',
      },
      textColor: {
        'light-primary': '#111827',
        'light-secondary': '#6b7280',
        'dark-primary': '#f1f5f9',
        'dark-secondary': '#cbd5e1',
      },
      borderColor: {
        'light': '#e5e7eb',
        'dark': '#334155',
      },
      transitionProperty: {
        'theme': 'background-color, border-color, color, fill, stroke',
      },
      transitionDuration: {
        'theme': '200ms',
      },
    },
  },
  plugins: [],
}
