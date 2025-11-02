import type { Config } from 'tailwindcss';

export const theme = {
  screens: {
    xs: '375px',
  },
  fontFamily: {
    sans: [
      'ui-sans-serif',
      'system-ui',
      'sans-serif',
      'Apple color Emoji',
      'Segoe UI emoji',
      'Segoe UI symbol',
      'Noto color Emoji',
    ],
  },
  colors: {},
  scale: {
    '98': '0.98',
  },
} satisfies Config['theme'];
