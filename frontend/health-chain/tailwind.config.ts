import type { Config } from "tailwindcss";

const config: Config = {
  content: [
    "./pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./components/**/*.{js,ts,jsx,tsx,mdx}",
    "./app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      colors: {
        brand: {
          black: "#000000",
          dark: "#0d0d0d",
          textBold: "#3c3c3c",
          navLine: "#4b4949",
          loginBtn: "#893d3d", 
          requestBtn: "#2c2525",
          footer: "#1e2833", 
        },
      },
      backgroundImage: {
        'blood-gradient': 'linear-gradient(135deg, #B32346 0%, #6a0b37 100%)',
      },
      boxShadow: {
        'blood-drop': '0px 4px 4px 0px rgba(165, 164, 164, 0.5)',
        'card': '0px 4px 20px rgba(0, 0, 0, 0.05)',
      },
      fontFamily: {
        poppins: ['var(--font-poppins)'],
        roboto: ['var(--font-roboto)'],
        manrope: ['var(--font-manrope)'],
        dmsans: ['var(--font-dm-sans)'],
      },
    },
  },
  plugins: [],
};
export default config;