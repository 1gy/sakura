/// <reference types="vitest" />
import { defineConfig } from "vite";

import build from "@hono/vite-cloudflare-pages";
import devServer from "@hono/vite-dev-server";
import adapter from "@hono/vite-dev-server/cloudflare";

export default defineConfig({
	server: {
		host: "0.0.0.0",
	},
	plugins: [
		build(),
		devServer({
			adapter,
			entry: "src/index.tsx",
		}),
	],
	test: {
		includeSource: ["src/**/*.ts"],
	},
	define: {
		"import.meta.vitest": "undefined",
	},
});
