import { config } from '@lpmini/config-eslint/index.js';
import ts from 'typescript-eslint';

export default ts.config(
  ...config,
  {
    ignores: ['.svelte-kit/*'],
  },

  {
    files: ['**/*.ts', '**/*.svelte'],
    rules: {
      // Allow no-undef because there are custom globals used
      'no-undef': 'off',
    },
  },

  {
    // Disable type-aware linting for files outside tsconfig
    files: ['e2e/**/*.ts', '**/vitest-setup-client.ts', '**/*.config.ts'],
    languageOptions: {
      parserOptions: {
        projectService: false,
      },
    },
    rules: {
      '@typescript-eslint/no-floating-promises': 'off',
    },
  },
);
