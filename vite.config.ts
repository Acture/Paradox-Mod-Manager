import {defineConfig} from "vite";
import react from "@vitejs/plugin-react";
// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST || '127.0.0.1';

// https://vitejs.dev/config/
export default defineConfig(async () => ({
	plugins: [react()],

	// Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
	//
	// 1. prevent vite from obscuring rust errors
	clearScreen: false,
	build: {
		rollupOptions: {
			external: ["/src/dev"]
		}
	},
	// 2. tauri expects a fixed port, fail if that port is not available
	server: {
		port: 1420,
		strictPort: true,
		host: host || false,
		hmr: host
				? {
					protocol: "ws",
					host,
					port: 1421,
				}
				: false,
		watch: {
			// 3. tell vite to ignore watching `src-tauri`
			ignored: ["**/src-tauri/**"],
		},
	},
}));
