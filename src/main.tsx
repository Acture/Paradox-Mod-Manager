import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import {ConfigProvider} from "antd"; // 引入 ConfigProvider 和 Ant Design 主题
import {themes} from "./theme.tsx";

const root = ReactDOM.createRoot(document.getElementById("root") as HTMLElement);
const selectedTheme = themes.dracula

if (process.env.NODE_ENV === "development") {
	const { DevSupport } = await import("@react-buddy/ide-toolbox");
	const { ComponentPreviews, useInitial } = await import("./dev");


	root.render(
			<React.StrictMode>
				<DevSupport
						ComponentPreviews={ComponentPreviews}
						useInitialHook={useInitial}
				>
					<ConfigProvider
							theme={selectedTheme}
					>
						<App />
					</ConfigProvider>
				</DevSupport>
			</React.StrictMode>
	);
} else {
	root.render(
			<React.StrictMode>
				<ConfigProvider
						theme={selectedTheme}
				>
					<App />
				</ConfigProvider>
			</React.StrictMode>
	);
}
