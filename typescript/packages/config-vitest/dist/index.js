"use strict";
var __spreadArray = (this && this.__spreadArray) || function (to, from, pack) {
    if (pack || arguments.length === 2) for (var i = 0, l = from.length, ar; i < l; i++) {
        if (ar || !(i in from)) {
            if (!ar) ar = Array.prototype.slice.call(from, 0, i);
            ar[i] = from[i];
        }
    }
    return to.concat(ar || Array.prototype.slice.call(from));
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.createCoverageConfig = createCoverageConfig;
exports.createNodeTestConfig = createNodeTestConfig;
exports.createBrowserTestConfig = createBrowserTestConfig;
exports.createSvelteTestConfig = createSvelteTestConfig;
/**
 * Base coverage configuration shared across all projects
 */
function createCoverageConfig(options) {
    if (options === void 0) { options = {}; }
    return {
        enabled: !!process.env.CI,
        provider: 'v8',
        reporter: ['text', 'json', 'html', 'json-summary'],
        reportsDirectory: './test-results/coverage',
        include: options.include || ['src/**/*.ts'],
        exclude: __spreadArray(['src/**/*.test.ts', 'src/**/*.spec.ts'], (options.exclude || []), true),
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
function createNodeTestConfig(options) {
    if (options === void 0) { options = {}; }
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
function createBrowserTestConfig(options) {
    if (options === void 0) { options = {}; }
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
function createSvelteTestConfig(options) {
    var _a;
    if (options === void 0) { options = {}; }
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
            exclude: __spreadArray(['src/**/*.stories.svelte'], (((_a = options.coverage) === null || _a === void 0 ? void 0 : _a.exclude) || []), true),
        }),
    };
}
