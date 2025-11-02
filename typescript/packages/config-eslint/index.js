import js from '@eslint/js';
import svelte from 'eslint-plugin-svelte';
import globals from 'globals';
import ts from 'typescript-eslint';
import importPlugin from 'eslint-plugin-import';
import unusedImports from 'eslint-plugin-unused-imports';

export const config = ts.config(
  js.configs.recommended,
  ...ts.configs.recommended,
  ...svelte.configs['flat/recommended'],
  {
    languageOptions: {
      globals: {
        ...globals.browser,
        ...globals.node,
      },
    },
  },

  // Global ignores
  {
    ignores: [
      'build/**',
      'dist/**',
      '.svelte-kit/**',
      '.turbo/**',
      '**/playwright-report/**',
      '**/blob-report/**',
      '**/playwright/**',
      '**/test-results/**',
      '**/test-report/**',
      '**/ci-test-artifacts/**',
    ],
  },

  // Test overrides
  {
    files: ['**/*.test.ts'],
    rules: {
      '@typescript-eslint/no-explicit-any': 'off',
      '@typescript-eslint/no-unused-vars': 'off',
    },
  },

  // Svelte overrides
  {
    files: ['**/*.svelte'],
    ignores: ['.svelte-kit/*'],
    languageOptions: {
      parserOptions: {
        parser: ts.parser,
      },
    },
  },

  // Import sorting and unused imports
  {
    plugins: {
      import: importPlugin,
      'unused-imports': unusedImports,
    },
    rules: {
      // Remove unused imports
      'unused-imports/no-unused-imports': 'error',
      'unused-imports/no-unused-vars': [
        'warn',
        {
          vars: 'all',
          varsIgnorePattern: '^_',
          args: 'after-used',
          argsIgnorePattern: '^_',
        },
      ],

      // Sort imports
      'import/order': [
        'error',
        {
          groups: [
            'builtin', // Node.js built-in modules
            'external', // External libraries
            'internal', // Internal modules (same repo)
            'parent', // Parent directories
            'sibling', // Same directory
            'index', // Index files
          ],
          'newlines-between': 'always',
          alphabetize: {
            order: 'asc',
            caseInsensitive: true,
          },
        },
      ],
    },
  },

  // Config files - allow any and skip type-aware linting
  {
    files: [
      '**/*.config.ts',
      '**/*.config.js',
      '**/*.config.mjs',
      '**/eslint.config.*',
      '**/vite.config.*',
      '**/vitest.config.*',
      '**/playwright.config.*',
      '**/svelte.config.*',
    ],
    rules: {
      '@typescript-eslint/no-explicit-any': 'off',
    },
  },

  // TypeScript and Svelte overrides
  {
    files: ['**/*.ts', '**/*.svelte'],
    ignores: [
      '**/*.config.ts',
      '**/*.config.js',
      '**/*.config.mjs',
      '**/eslint.config.*',
      '**/vite.config.*',
      '**/vitest.config.*',
      '**/playwright.config.*',
      '**/svelte.config.*',
    ],
    rules: {
      // Allow explicit any
      '@typescript-eslint/no-explicit-any': 'off',

      // Disable default unused vars rule in favor of unused-imports plugin
      '@typescript-eslint/no-unused-vars': 'off',

      // Consistent type imports
      '@typescript-eslint/consistent-type-imports': 'error',

      // Object shorthand properties
      'object-shorthand': ['error', 'always'],

      // Require promises to be awaited or handled
      '@typescript-eslint/no-floating-promises': 'error',
    },
    languageOptions: {
      parserOptions: {
        ecmaVersion: 'latest',
        sourceType: 'module',
        projectService: true,
        extraFileExtensions: ['.svelte'],
      },
    },
    settings: {
      'import/resolver': {
        typescript: true,
        node: true,
      },
    },
  },
);
