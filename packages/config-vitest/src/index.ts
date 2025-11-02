import type { UserConfig } from 'vitest/config';

/**
 * Shared Vitest configuration for the LP Mini monorepo.
 * @packageDocumentation
 */

type CoverageConfig = NonNullable<NonNullable<UserConfig['test']>['coverage']>;

/**
 * Base coverage configuration shared across all projects
 */
export function createCoverageConfig(
  options: {
    include?: string[];
    exclude?: string[];
  } = {},
): CoverageConfig {
  return {
    enabled: !!process.env.CI,
    provider: 'v8',
    reporter: ['text', 'json', 'html', 'json-summary'],
    reportsDirectory: './test-results/coverage',
    include: options.include || ['src/**/*.ts'],
    exclude: ['src/**/*.test.ts', 'src/**/*.spec.ts', ...(options.exclude || [])],
    all: true,
    thresholds: {
      lines: 0,
      functions: 0,
      branches: 0,
      statements: 0,
    },
  };
}

/**
 * Base test configuration for Node.js projects
 */
export function createNodeTestConfig(
  options: {
    coverage?: {
      include?: string[];
      exclude?: string[];
    };
  } = {},
): UserConfig['test'] {
  return {
    environment: 'node',
    reporters: process.env.CI ? ['verbose', 'json', 'html', 'junit'] : ['default'],
    outputFile: {
      junit: './test-results/junit.xml',
      json: './test-results/test-results.json',
      html: './test-results/index.html',
    },
    coverage: createCoverageConfig(options.coverage),
  };
}

/**
 * Test configuration for browser/jsdom projects (e.g., UI components)
 */
export function createBrowserTestConfig(
  options: {
    setupFiles?: string[];
    coverage?: {
      include?: string[];
      exclude?: string[];
    };
  } = {},
): UserConfig['test'] {
  return {
    globals: true,
    css: false,
    environment: 'jsdom',
    setupFiles: options.setupFiles || [],
    reporters: process.env.CI ? ['verbose', 'json', 'html', 'junit'] : ['default'],
    outputFile: {
      junit: './test-results/junit.xml',
      json: './test-results/test-results.json',
      html: './test-results/index.html',
    },
    coverage: createCoverageConfig(options.coverage),
  };
}

/**
 * Test configuration for Svelte component libraries
 */
export function createSvelteTestConfig(
  options: {
    setupFiles?: string[];
    include?: string[];
    coverage?: {
      include?: string[];
      exclude?: string[];
    };
  } = {},
): UserConfig['test'] {
  return {
    globals: true,
    css: false,
    environment: 'jsdom',
    setupFiles: options.setupFiles || [],
    include: options.include || ['src/**/*.test.ts'],
    passWithNoTests: true,
    reporters: process.env.CI ? ['verbose', 'json', 'html', 'junit'] : ['default'],
    outputFile: {
      junit: './test-results/junit.xml',
      json: './test-results/test-results.json',
      html: './test-results/index.html',
    },
    coverage: createCoverageConfig({
      include: ['src/**/*.{ts,svelte}'],
      exclude: ['src/**/*.stories.svelte', ...(options.coverage?.exclude || [])],
    }),
  };
}
