/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        gv: {
          primary: '#00FF88',
          secondary: '#00CCFF',
          dark: '#0D1117',
          darker: '#010409',
          surface: '#161B22',
          border: '#30363D',
        }
      },
      animation: {
        'pulse-slow': 'pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite',
        'glow': 'glow 2s ease-in-out infinite alternate',
      },
      keyframes: {
        glow: {
          '0%': { boxShadow: '0 0 5px #00FF88, 0 0 10px #00FF88' },
          '100%': { boxShadow: '0 0 15px #00FF88, 0 0 30px #00FF88' },
        }
      }
    },
  },
  plugins: [],
}
