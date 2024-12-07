import globals from "globals";
import pluginJs from "@eslint/js";
import prettierPlugin from "eslint-plugin-prettier";

export default [
  pluginJs.configs.recommended, // Include ESLint recommended rules
  {
    languageOptions: {
      globals: globals.browser,
    },
    plugins: {
      prettier: prettierPlugin, // Add Prettier as a plugin
    },
    rules: {
      "prettier/prettier": "error", // Report Prettier formatting issues as ESLint errors
    },
  },
];
