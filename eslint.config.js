import globals from "globals";
import pluginJs from "@eslint/js";
import tseslint from "typescript-eslint";
import pluginReact from "eslint-plugin-react";


/** @type {import('eslint').Linter.Config[]} */
export default tseslint.config(
	{
		files: ["**/*.{js,mjs,cjs,ts,jsx,tsx}"]
	},
	{
		languageOptions: {
			globals: {...globals.browser, ...globals.node},
			parserOptions: {
				projectService: true
			}
		}
	},
	{
		rules: {
			"react/react-in-jsx-scope": "off",
			"react/jsx-uses-react": "off"
		}
	},
	pluginJs.configs.recommended,
	...tseslint.configs.stylisticTypeChecked,
	pluginReact.configs.flat.recommended,
);