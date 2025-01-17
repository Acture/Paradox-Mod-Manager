import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import {DevSupport} from "@react-buddy/ide-toolbox";
import {ConfigProvider, theme} from "antd"; // 引入 ConfigProvider 和 Ant Design 主题

import {ComponentPreviews, useInitial} from "./dev";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
		<React.StrictMode>
			<ConfigProvider
					theme={{
						token: {
							// 背景色相关
							colorBgLayout: "#282a36", // 全局布局背景
							colorBgContainer: "#44475a", // 容器背景（如 Card/Modal）
							colorBgElevated: "#44475a", // 浮动容器背景

							// 文本颜色
							colorText: "#f8f8f2", // 普通文本颜色
							colorTextSecondary: "#6272a4", // 次要文本颜色

							// 主色和组件颜色
							colorPrimary: "#bd93f9", // 全局主色（按钮、选中状态）
							colorLink: "#ff79c6", // 链接颜色
							colorSuccess: "#50fa7b", // 成功状态颜色
							colorWarning: "#f1fa8c", // 警告状态颜色
							colorError: "#ff5555", // 错误状态颜色

							// 边框颜色
							colorBorder: "#6272a4", // 边框颜色
							colorBorderSecondary: "#44475a", // 次要边框颜色

						},
						algorithm: theme.defaultAlgorithm, // 可切换为暗色模式
					}}
			>

				<DevSupport ComponentPreviews={ComponentPreviews}
				            useInitialHook={useInitial}
				>
					<App/>
				</DevSupport>
			</ConfigProvider>

		</React.StrictMode>,
);
