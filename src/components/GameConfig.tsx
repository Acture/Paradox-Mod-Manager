import React, {useEffect, useState} from "react";
import {Card, Button, Flex, Row, Col, message} from "antd";
import {open} from '@tauri-apps/plugin-dialog';
import { warn, debug, trace, info, error } from '@tauri-apps/plugin-log';
import {invoke} from "@tauri-apps/api/core";

interface GameConfigProps {
	game_name: string;
	game_dir: string | null;
	mod_dir: string | null;
}

async function setup_game_config(game_name: string, game_dir: string, mod_dir: string) {
	await invoke("setup_game_config", {game_name, game_dir, mod_dir});
}

async function read_game_config(game_name: string) : Promise<GameConfigProps> {
	const result = await invoke("read_game_config", {game_name});
	const config = result as GameConfigProps;
	if (config && typeof config === 'object') {
		return config;
	}
	throw new Error(`Failed to read ${game_name} config`);
}




const GameConfig: React.FC<GameConfigProps> = ({game_name, game_dir, mod_dir}) => {
	const [_game_dir, setGameDir] = useState<string | null>(game_dir);
	const [_mod_dir, setModDir] = useState<string | null>(mod_dir);
	const [messageApi, contentHolder] = message.useMessage();

	useEffect(() => {
		read_game_config(game_name).then((config: GameConfigProps) => {
			setGameDir(config.game_dir);
			setModDir(config.mod_dir);
			messageApi.success(`${game_name} Config Loaded`);
		}).catch((error: any) => {
			messageApi.error(`Failed to get ${game_name} Config`);
			error(`Failed to get ${game_name} Config: ${error}`);
		});
	}, []);

	// 设置 `game_dir` 的函数
	const handleSetGameDir = async () => {
		const selected_dir = await open({
			multiple: false,
			directory: true,
			title: `Set directory for ${game_name}`,
		});

		if (selected_dir) {
			setGameDir(selected_dir as string); // 更新组件内的状态
		}
	};

	// 设置 `mod_dir` 的函数
	const handleSetModDir = async () => {
		const selected_dir = await open({
			multiple: false,
			directory: true,
			title: `Set Mod directory for ${game_name}`,
		});
		info(`selected_dir: ${selected_dir}`);
		if (selected_dir) {
			setModDir(selected_dir as string); // 更新组件内的状态
		}
	};

	const handleSave = async () => {
		if (!_game_dir){
			messageApi.error(`Please set ${game_name} game directory`);
			return;
		}
		if (!_mod_dir){
			messageApi.error(`Please set ${game_name} mod directory`);
			return;
		}
		await setup_game_config(game_name, _game_dir, _mod_dir).then(
				()=>{messageApi.success(`${game_name} Config Saved`)}
		).catch(
				(error)=>{
					messageApi.error(`Failed to save ${game_name} Config`)
					error(`Failed to save ${game_name} Config: ${error}`)
				}
		)
	};


	return (
			<Card
					type="inner"
					title={(<Flex justify={"space-between"} align={"center"}>
						<span>Directory Config</span>
						<Button color={"blue"} variant={"solid"} style={{boxShadow: "none"}}
						        onClick={handleSave}>Save</Button>
					</Flex>)}


			>
				{contentHolder}
				<Row justify={"space-around"} align={"middle"}>
					<Col flex={2}>
						<h4>Game Directory</h4>
					</Col>
					<Col flex={1}>
						<Button onClick={handleSetGameDir}>SET</Button>
					</Col>
					<Col flex={2}>
						<h4>Mod Directory</h4>
					</Col>
					<Col flex={1}>
						<Button onClick={handleSetModDir}>SET</Button>
					</Col>
				</Row>
				<Row justify={"space-around"} align={"middle"}>
					<Col span={12}>
						<span
								style={{
									color: _game_dir ? "inherit" : "gray",
									fontWeight: _game_dir ? "inherit" : "bold"
								}}
						>
							{_game_dir ? _game_dir : "NOT SET"}
						</span>
					</Col>
					<Col span={12}>
						<span
								style={{
									color: _mod_dir ? "inherit" : "gray",
									fontWeight:  _mod_dir? "inherit" : "bold"
								}}>
							{_mod_dir ? _mod_dir : "NOT SET"}
						</span>
					</Col>
				</Row>

			</Card>

	);
};

export default GameConfig;
