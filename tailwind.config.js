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
        move: {
          "0%, 100%": { transform: "translateX(0)" },
          "50%": { transform: "translateX(-200px)" },
        },
        wiggle: {
          "0%, 100%": { transform: "rotate(-3deg)" },
          "50%": { transform: "rotate(3deg)" },
        },
      },
      animation: {
        "spin-fast": "spin 0.5s linear infinite",
        move: "move 8s infinite linear",
        wiggle: "wiggle 0.25s infinite",
      },
    },
  },
  plugins: [],
};
