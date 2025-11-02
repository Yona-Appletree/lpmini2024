import path from 'node:path';

import type { StorybookConfig } from '@storybook/sveltekit';

import { processIndexerResults } from '../src/lib/monorepo-indexer';

/**
 * This function is used to resolve the absolute path of a package.
 * It is needed in projects that use Yarn PnP or are set up within a monorepo.
 */
function getAbsolutePath(value: string): string {
  return path.dirname(require.resolve(path.join(value, 'package.json')));
}

const config: StorybookConfig = {
  stories: [
    '../src/**/*.mdx',
    '../../control/src/**/*.stories.@(js|ts|svelte)',
    '../../../packages/**/src/**/*.stories.@(js|ts|svelte)',
  ],
  addons: [
    getAbsolutePath('@storybook/addon-svelte-csf'),
    getAbsolutePath('@storybook/addon-a11y'),
    getAbsolutePath('@storybook/addon-docs'),
    getAbsolutePath('@storybook/addon-vitest'),
  ],
  framework: {
    name: getAbsolutePath('@storybook/sveltekit'),
    options: {},
  },
  experimental_indexers: async (existingIndexers) => processIndexerResults(existingIndexers || []),
};
export default config;
