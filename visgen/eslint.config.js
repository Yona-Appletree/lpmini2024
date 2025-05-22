import js from "@eslint/js";
import globals from "globals";
import reactHooksPlugin from "eslint-plugin-react-hooks";
import reactRefreshPlugin from "eslint-plugin-react-refresh";
import tseslint from "typescript-eslint";
import importPlugin from "eslint-plugin-import";
import unusedImportsPlugin from "eslint-plugin-unused-imports";
import promisePlugin from "eslint-plugin-promise";

export default tseslint.config(
  { ignores: ["dist"] },
  {
    extends: [
      js.configs.recommended,
      ...tseslint.configs.recommended,
      importPlugin.flatConfigs.recommended,
      importPlugin.flatConfigs.typescript,
    ],
    files: ["**/*.{ts,tsx}"],
    languageOptions: {
      ecmaVersion: 2020,
      globals: globals.browser,
    },
    plugins: {
      "react-hooks": reactHooksPlugin,
      "react-refresh": reactRefreshPlugin,
      "unused-imports": unusedImportsPlugin,
      promise: promisePlugin,
    },
    rules: {
      // -----------------------------------------------------------------------
      // React
      //
      ...reactHooksPlugin.configs.recommended.rules,
      "react-refresh/only-export-components": [
        "warn",
        { allowConstantExport: true },
      ],

      // -----------------------------------------------------------------------
      // Unused imports & variables
      //
      "no-unused-vars": "off",
      "@typescript-eslint/no-unused-vars": "off",
      "unused-imports/no-unused-imports": "error",
      "unused-imports/no-unused-vars": [
        "error",
        { args: "after-used", argsIgnorePattern: "^_" },
      ],

      // -----------------------------------------------------------------------
      // Imports
      //
      "sort-imports": [
        "error",
        {
          ignoreCase: true,
          ignoreDeclarationSort: true,
        },
      ],

      "import/no-unresolved": "off",
    },
  },

  // ---------------------------------------------------------------------------
  // Disable react export rule for shadcn-ui
  //
  // See https://github.com/shadcn-ui/ui/issues/1534
  //
  {
    files: ["**/ui/**/*.ts", "**/ui/**/*.tsx", "**/tools/theme-provider.tsx"],
    rules: {
      "react-refresh/only-export-components": "off",
    },
  },
);
