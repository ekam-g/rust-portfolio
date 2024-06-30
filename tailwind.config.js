/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["./templates/**/*.{html,hbs}"],
    theme: {
      animation: [
        'fadeIn { animation: fadeIn 1s; }',
        '@keyframes fadeIn { 0% { opacity: 0; } 100% { opacity: 1; } }'
      ],
      extend: {},
    },
    plugins: [],
  }