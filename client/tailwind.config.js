/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        // Discord-like colors
        'discord-dark': '#202225',
        'discord-darker': '#2f3136',
        'discord-darkest': '#292b2f',
        'discord-sidebar': '#36393f',
        'discord-channel-default': '#96989d',
        'discord-channel-hover': '#dcddde',
        'discord-blurple': '#5865f2',
        'discord-blurple-hover': '#4752c4',
        'discord-green': '#3ba55d',
        'discord-yellow': '#faa81a',
        'discord-red': '#ed4245',
        'discord-link': '#00b0f4',
        'discord-mention': '#5865f2',
      },
      fontFamily: {
        'sans': ['gg sans', 'Noto Sans', 'Helvetica Neue', 'Helvetica', 'Arial', 'sans-serif'],
      },
    },
  },
  plugins: [],
}
