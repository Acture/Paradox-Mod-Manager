import { ThemeConfig, theme } from "antd";

// 浅色主题
export const lightTheme: ThemeConfig = {
	token: {
		colorBgLayout: "#f0f2f5", // 浅色背景
		colorBgContainer: "#ffffff", // 容器背景
		colorBgElevated: "#ffffff", // 浮动容器背景
		colorText: "#000000", // 文本颜色
		colorTextSecondary: "#555555", // 次要文本颜色
		colorBorder: "#d9d9d9", // 边框颜色
		colorPrimary: "#1890ff", // 主色调
		colorLink: "#1890ff", // 链接颜色
		colorSuccess: "#52c41a", // 成功状态颜色
		colorWarning: "#faad14", // 警告色
		colorError: "#ff4d4f", // 错误状态颜色
	},
	algorithm: theme.defaultAlgorithm, // 浅色模式
};

// 深色主题
export const darkTheme: ThemeConfig = {
	token: {
		colorBgLayout: "#282a36", // 深色背景
		colorBgContainer: "#44475a", // 容器背景
		colorBgElevated: "#44475a", // 浮动容器背景
		colorText: "#f8f8f2", // 文本颜色
		colorTextSecondary: "#6272a4", // 次要文本颜色
		colorBorder: "#6272a4", // 边框颜色
		colorPrimary: "#bd93f9", // 主色调
		colorLink: "#ff79c6", // 链接颜色
		colorSuccess: "#50fa7b", // 成功状态颜色
		colorWarning: "#f1fa8c", // 警告色
		colorError: "#ff5555", // 错误状态颜色
	},
	algorithm: theme.darkAlgorithm, // 深色模式
};

// 自定义主题，用户可根据需求加入更多主题
export const customTheme: ThemeConfig = {
	token: {
		colorBgLayout: "#003366",
		colorBgContainer: "#006699",
		colorBgElevated: "#005577",
		colorText: "#ffffff",
		colorTextSecondary: "#cccccc",
		colorPrimary: "#00ccff",
		colorLink: "#0066ff",
		colorSuccess: "#00cc66",
		colorWarning: "#ffcc00",
		colorError: "#ff3300",
		colorBorder: "#004466",
	},
	algorithm: theme.defaultAlgorithm,
};


export const draculaTheme: ThemeConfig = {
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
}

// 所有主题的集合
export const themes = {
	light: lightTheme,
	dark: darkTheme,
	custom: customTheme,
	dracula: draculaTheme,
};