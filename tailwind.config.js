/** @type {import('tailwindcss').Config} */

// This needs to be kept in sync with what's in `src/dialogs.tsx`.
const colors =
  "slate|orange|amber|emerald|teal|cyan|sky|blue|indigo|violet|purple|fuchsia|rose";

// This needs to be kept in sync with what's in `src/nodes.tsx`.
const backgrounds = [
  // Node backgrounds
  200, 300, 400,
  // Node header
  500,
  // Handle background
  700,
];

module.exports = {
  content: ["./src/**/*.{html,tsx,js}"],
  theme: {
    extend: {},
  },
  plugins: [
    require("@tailwindcss/forms"),
    require("@tailwindcss/aspect-ratio"),
  ],
  safelist: [
    {
      pattern: new RegExp(`bg-(${colors})-(${backgrounds.join("|")})`),
    },
    {
      pattern: new RegExp(`bg-(${colors})-100`),
      variants: ["hover"],
    },
    {
      pattern: new RegExp(`border-(${colors})-300`),
    },
    {
      pattern: new RegExp(`text-(${colors})-500`),
    },
    {
      pattern: new RegExp(`(border|ring)-(${colors})-600`),
    },
    {
      pattern: new RegExp(`ring-offset-(${colors})-700`),
    },
  ],
};
