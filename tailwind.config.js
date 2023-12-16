/** @type {import('tailwindcss').Config}*/
const config = {
  content: ['./src/**/*.{html,js,svelte,ts}', './src/**/**/*.{html,js,svelte,ts}', "./index.html",],

  theme: {
    extend: {
      colors: {
        "gradient-start": "#2E4B73",
        "gradient-end": "#1D1D49",
        "main" : "#0F172A"
      },
      fontFamily: {
        "dm-sans": ['DM Sans'],
        "poppins": ["Poppins"]
      }
    }
  },

  plugins: []
};

module.exports = config;
