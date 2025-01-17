import {useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {App as AntdApp, Button, Select, Layout, Card, Flex} from "antd"; // 引入 Antd App 和组件
import {EditOutlined, EllipsisOutlined, SettingOutlined} from '@ant-design/icons';

import "./App.css";

async function setup_game_config(game_name: string, game_dir: string, mod_dir: string) {
	await invoke("setup_game_config", {game_name, game_dir, mod_dir});
}

enum Game {
	EU4 = "EU4",
	CK3 = "CK3",
	HOI4 = "HOI4",
	VIC2 = "VIC2",
}

const default_img_link = "https://images.ctfassets.net/u73tyf0fa8v1/rFUb3YH3gQZfH2ehUiR6n/ed70f8f3919e0797d070be80f04415a2/pdx-logo-main.png?w=640&q=75"

const GameOptions = [
	{
		value: Game.EU4,
		label: "Europa Universalis IV",
		img_url: "https://images.ctfassets.net/u73tyf0fa8v1/6sw36JpxucfjPL99oMz8Ed/40a2d434b7a5955629847363e46f1452/euiv-brands.jpg?w=1080&q=75"
	},
	{value: Game.CK3, label: "Crusader Kings III"},
	{value: Game.HOI4, label: "Hearts of Iron IV"},
	{value: Game.VIC2, label: "Victoria II"},
]

const actions: React.ReactNode[] = [
	<EditOutlined key="edit" style={{color: "white"}}/>,
	<SettingOutlined key="setting" style={{color: "white"}}/>,
	<EllipsisOutlined key="ellipsis" style={{color: "white"}}/>,
];

function App() {


	return (
			<AntdApp style={{width: "100vw", height: "100vh", margin: 0}}>
				<Layout style={{width: "100%", height: "100%"}}>
					<Layout.Sider style={{width: "100%", height: "100%", overflowY: "auto", direction: "rtl"}}>
						<Flex vertical style={{width: "100%", height: "auto", direction:"ltr"}}>
							{
								GameOptions.map(({value, label, img_url}) => (
										<Card
												style={{width: "100%", height: "auto"}}
												type="inner"
												size={"small"}
												hoverable={true}
												bordered={false}
												cover={<img
														alt={label}
														src={img_url ? img_url : default_img_link}
														style={{height: "20%", objectFit: "cover"}}

												/>}

												actions={actions}
										>
											<Card.Meta
													title={label}></Card.Meta>
										</Card>
								))
							}
							<Button></Button>
						</Flex>
					</Layout.Sider>

				</Layout>

			</AntdApp>
	);
}

export default App;
