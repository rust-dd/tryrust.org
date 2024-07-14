/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      backgroundImage: {
        "custom-radial":
          "radial-gradient(37.4% 50% at 75.5% 50%, rgba(255, 239, 92, 0.2) 0%, rgba(255, 239, 92, 0) 100%)",
      },
      keyframes: {
        blink: {
          "0%, 100%": { opacity: 1 },
          "50%": { opacity: 0 },
        },
      },
      animation: {
        blink: "blink 1s step-end infinite",
      },
    },
  },
  plugins: [],
};
