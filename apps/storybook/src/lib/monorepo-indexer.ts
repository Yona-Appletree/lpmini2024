import type { Indexer } from 'storybook/internal/types';

/**
 * Function to process existing indexer results and modify titles
 */
export async function processIndexerResults(existingIndexers: Indexer[]): Promise<Indexer[]> {
  return existingIndexers.map((indexer) => ({
    ...indexer,
    createIndex: async (fileName, options) => {
      // Let the original indexer run first
      const results = await indexer.createIndex(fileName, options);

      // Check if this is a story file we want to modify
      if (fileName.match(/\.stories\.(js|ts|svelte)$/)) {
        // Normalize the path to use forward slashes
        const normalizedPath = fileName.replace(/\\/g, '/');

        // The fileName should already be relative to the monorepo root based on the stories config
        // But let's ensure we have the correct relative path by finding the monorepo structure
        let relativePath = normalizedPath;

        // If the path contains the monorepo structure, extract the relative part
        const packageMatch = normalizedPath.match(/.*\/packages\/([^/]+)\/(.+)$/);
        if (packageMatch) {
          relativePath = `packages/${packageMatch[1]}/${packageMatch[2]}`;
        } else {
          const appMatch = normalizedPath.match(/.*\/apps\/([^/]+)\/(.+)$/);
          if (appMatch) {
            relativePath = `apps/${appMatch[1]}/${appMatch[2]}`;
          }
        }

        // Get the custom title for this file
        const customTitle = pathToTitle(relativePath);

        // Modify the results to use our custom title
        return results.map((result) => ({
          ...result,
          title: customTitle,
        }));
      }

      return results;
    },
  }));
}

/**
 * Character to use in story names that looks like a slash so we can have a
 * flat story structure which is easier to navigate.
 *
 * Using an actual slash char would cause StoryBook to nest the stories, which we
 * don't want.
 */
export const dirTitleSeparator = ' ∕ ';

/**
 * Converts a file path to a hierarchical title array for the monorepo structure
 * @param relativePath - The relative path from the monorepo root
 * @returns Array of title segments
 */
export function pathToTitle(relativePath: string): string {
  return buildSegments(relativePath).join(' / ');
}

export function buildSegments(relativePath: string): string[] {
  // Handle packages
  if (relativePath.startsWith('packages/')) {
    const packageMatch = relativePath.match(/^packages\/([^/]+)\/src\/(.+)$/);
    if (packageMatch) {
      const [, packageName, restPath] = packageMatch;
      const componentPath = restPath.replace(/\.stories\.(js|ts|svelte)$/, '');
      return [packageName, componentPath.split('/').filter(Boolean).join(dirTitleSeparator)];
    }
  }

  // Handle apps
  if (relativePath.startsWith('apps/')) {
    const appMatch = relativePath.match(/^apps\/([^/]+)\/src\/(.+)$/);
    if (appMatch) {
      const [, appName, restPath] = appMatch;
      const cleanPath = restPath.replace(/\.stories\.(js|ts|svelte)$/, '');

      // Check if it's a route (starts with routes/)
      if (cleanPath.startsWith('routes/')) {
        // Remove "routes/" prefix and "page" suffix
        let routePath = cleanPath.replace(/^routes\//, '').replace(/\/page$/, '');

        // Check if this is a root route (ends with just the route group)
        const isRootRoute = routePath.match(/^\([^)]+\)\/?$/);

        // Clean up route groups like (public) -> public
        routePath = routePath.replace(/\(([^)]+)\)/g, '$1');

        // Split by / and filter out empty parts
        const routeParts = routePath.split('/').filter(Boolean);

        // If this is a root route, add <root> to the parts
        if (isRootRoute && routeParts.length > 0) {
          routeParts[routeParts.length - 1] += dirTitleSeparator + '<root>';
        }

        return [appName + ' routes', routeParts.join(dirTitleSeparator)];
      }
      // Check if it's a component (starts with lib/)
      else if (cleanPath.startsWith('lib/')) {
        // Remove "lib/components/" prefix if present, otherwise fallback to removing "lib/"
        let componentPath = cleanPath.replace(/^lib\/components\//, '');
        if (componentPath === cleanPath) {
          componentPath = cleanPath.replace(/^lib\//, '');
        }

        // Split by / and filter out empty parts
        const componentParts = componentPath.split('/').filter(Boolean);
        return [appName + ' components', componentParts.join(dirTitleSeparator)];
      }
      // Fallback for other paths
      else {
        const otherParts = cleanPath.split('/').filter(Boolean);
        return [appName, otherParts.join(dirTitleSeparator)];
      }
    }
  }

  // Fallback for other paths
  return relativePath
    .replace(/\.stories\.(js|ts|svelte)$/, '')
    .split('/')
    .filter(Boolean);
}
