import type { Linter } from "eslint";
import tsPlugin from "@typescript-eslint/eslint-plugin";
import tsParser from "@typescript-eslint/parser";
import reactPlugin from "eslint-plugin-react";

const tseslint = tsPlugin as unknown as any;
const reacteslint = reactPlugin as unknown as any;

const config: Linter.Config[] = [
  {
    files: ["**/*.ts", "**/*.tsx"],
    plugins: {
      "@typescript-eslint": tseslint,
      react: reacteslint,
    },
    languageOptions: {
      parser: tsParser,
      parserOptions: {
        ecmaVersion: 2022,
        sourceType: "module",
        ecmaFeatures: {
          jsx: true,
        },
      },
    },
    rules: {
      quotes: ["error", "single"],
      semi: ["error", "always"],
    },
  },
  // JavaScript/JSX用の設定は同様
];

export default config;
