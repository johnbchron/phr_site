/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [ "./crates/**/*.rs" ],
  theme: {
    fontFamily: {
			'sans': [
        'inter', 'ui-sans-serif', 'system-ui', 'sans-serif', "Apple Color Emoji",
        "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji"
      ],
    },
    screens: {
      'xs': '480px',
      'sm': '640px',
      'md': '768px',
      'lg': '1024px',
    },
    extend: {
      boxShadow: {
        custom: "6px 6px 0px 0px #000000",
      },
    },
  },
  plugins: [],
}
