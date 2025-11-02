# LP Mini - TypeScript Monorepo

This is a Turborepo-based monorepo for the LP Mini LED control system.

## Structure

```
typescript/
├── apps/
│   ├── control/       # SvelteKit LED control application
│   └── storybook/     # Storybook component documentation
├── packages/
│   └── config-eslint/ # Shared ESLint configuration
└── scripts/           # Shared scripts
```

## Getting Started

### Prerequisites

- Node.js >= 22
- pnpm >= 10

### Installation

```bash
pnpm install
```

## Available Scripts

### Development

```bash
# Run all apps in dev mode
pnpm turbo dev

# Run specific app
pnpm --filter @lpmini/control dev
pnpm --filter @lpmini/storybook dev
```

### Code Quality

```bash
# Check code quality (linting, formatting, type checking)
pnpm turbo check

# Fix auto-fixable issues
pnpm turbo fix

# Format code
pnpm fix:prettier

# Check formatting
pnpm check:prettier
```

### Testing

```bash
# Run all tests
pnpm turbo test

# Run e2e tests
pnpm turbo e2e
```

### Building

```bash
# Build all apps
pnpm turbo build
```

### Pre-commit & Validation

```bash
# Run pre-commit checks (fast)
pnpm turbo pre-commit

# Run full validation (recommended before pushing)
pnpm turbo validate
```

## Dependency Management

This project uses [syncpack](https://github.com/JamieMason/syncpack) to manage dependencies across packages:

```bash
# Check for version mismatches
pnpm check:syncpack

# Fix version mismatches
pnpm fix:syncpack-mismatches

# Format package.json files
pnpm fix:syncpack-format
```

## Configuration

- **ESLint**: Shared config in `packages/config-eslint`
- **Prettier**: `.prettierrc`
- **Syncpack**: `.syncpackrc.json`
- **Turbo**: `turbo.jsonc`
- **Lint-staged**: `.lintstagedrc.all.json` and `.lintstagedrc.fast.json`

## Apps

### Control

SvelteKit application for controlling LED displays.

- **Path**: `apps/control`
- **Package**: `@lpmini/control`
- **Dev**: `pnpm --filter @lpmini/control dev`

### Storybook

Component documentation and testing environment.

- **Path**: `apps/storybook`
- **Package**: `@lpmini/storybook`
- **Dev**: `pnpm --filter @lpmini/storybook dev`

## Packages

### config-eslint

Shared ESLint configuration for the monorepo.

- **Path**: `packages/config-eslint`
- **Package**: `@lpmini/config-eslint`
