import { describe, expect, it } from 'vitest';

import { buildSegments, dirTitleSeparator, pathToTitle } from './monorepo-indexer';

describe('monorepo-indexer', () => {
  describe('pathToTitle', () => {
    it('should convert package path to title', () => {
      const result = pathToTitle('packages/config-eslint/src/index.stories.ts');
      expect(result).toBe('config-eslint / index');
    });

    it('should convert app component path to title', () => {
      const result = pathToTitle('apps/control/src/lib/Button.stories.svelte');
      expect(result).toBe('control components / Button');
    });

    it('should convert app route path to title', () => {
      const result = pathToTitle('apps/control/src/routes/page.stories.svelte');
      expect(result).toBe('control routes / page');
    });
  });

  describe('buildSegments', () => {
    it('should build segments for package files', () => {
      const result = buildSegments('packages/config-eslint/src/index.stories.ts');
      expect(result).toEqual(['config-eslint', 'index']);
    });

    it('should build segments for app components', () => {
      const result = buildSegments('apps/control/src/lib/Button.stories.ts');
      expect(result).toEqual(['control components', 'Button']);
    });

    it('should use custom separator in segments', () => {
      const result = buildSegments('apps/control/src/lib/components/nested/Button.stories.ts');
      expect(result[1]).toContain(dirTitleSeparator);
    });
  });
});
