import { fileURLToPath } from 'node:url';

import { includeIgnoreFile } from '@eslint/compat';
import { config } from '@lpmini/config-eslint';
import storybook from 'eslint-plugin-storybook';
import ts from 'typescript-eslint';

const gitignorePath = fileURLToPath(new URL('../../.gitignore', import.meta.url));

export default ts.config(
  includeIgnoreFile(gitignorePath),
  ...config,
  ...storybook.configs['flat/recommended'],
  {
    ignores: ['scripts/**', '.storybook/**/*.js'],
  },
  {
    rules: {
      'unicorn/no-empty-file': 'off',
    },
  },
  {
    // Disable type-aware linting for .storybook config files
    files: ['.storybook/**/*.ts'],
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
