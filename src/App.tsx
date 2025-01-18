import {App as AntdApp, Button, Layout, Tabs} from "antd"; // 引入 Antd App 和组件
import {FolderAddOutlined} from '@ant-design/icons';
import pdx_logo from "./assets/pdx_logo.png";
import eu4_main_background from "./assets/eu4_main_bg.jpg";
import eu4_logo from "./assets/eu4_logo.png";
import ck3_main_background from "./assets/ck3_main_bg.jpg";
import ck3_logo from "./assets/ck3_logo.png";
import hoi4_main_background from "./assets/hoi4_main_bg.jpg";
import hoi4_logo from "./assets/hoi4_logo.png";
import stellaris_main_background from "./assets/stellaris_main_bg.jpg";
import stellaris_logo from "./assets/stellaris_logo.png";
import vic3_main_background from "./assets/vic3_main_bg.jpg";
import vic3_logo from "./assets/vic3_logo.png";
import "./App.css";
import GameTab from "./components/GameTab.tsx";
import GameContent from "./components/GameContent.tsx";


interface Game {
	game_name: string;
	game_dir: string | null;
	mod_dir: string | null;
	bg_img: string | null;
	logo_img: string | null;
}

const createNewGame = (overrides?: Partial<Game>): Game => {
	return {
		game_name: overrides?.game_name || "Untitled Game",
		game_dir: overrides?.game_dir || null,
		mod_dir: overrides?.mod_dir || null,
		bg_img: overrides?.bg_img || null,
		logo_img: overrides?.logo_img || null
	};
};


const GameList: Game[] = [
	createNewGame({
		game_name: "Europa Universalis IV",
		bg_img: eu4_main_background,
		logo_img: eu4_logo,
	}),
	createNewGame({
		game_name: "Crusader Kings III",
		bg_img: ck3_main_background,
		logo_img: ck3_logo,
	}),
	createNewGame({
		game_name: "Hearts of Iron IV",
		bg_img: hoi4_main_background,
		logo_img: hoi4_logo,
	}),
	createNewGame({
		game_name: "Stellaris",
		bg_img: stellaris_main_background,
		logo_img: stellaris_logo,
	}),
	createNewGame({
		game_name: "Victoria II",
		bg_img: vic3_main_background,
		logo_img: vic3_logo,
	}),

];


const GenerateTabs = (GameList: Game[], default_img: string) => {
	const gameTabs = GameList.map(({game_name, bg_img, game_dir, mod_dir, logo_img}) => {
		return {
			key: game_name,
			label: (
					<GameTab game_name={game_name} img_src={bg_img || default_img}/>
			),
			children: (

					<GameContent game_name={game_name} game_dir={game_dir} mod_dir={mod_dir}
					             game_logo={logo_img}/>
			)
		};
	});
	const addGameTab = {
		key: "Add Game",
		label: (
				<Button
						size="large"
						style={{
							width: "200px",
							height: "100px",
						}}
						icon={<FolderAddOutlined/>}

				/>
		),
		children: ""
	};
	return [...gameTabs, addGameTab];
};


function App() {


	return (
			<AntdApp style={{width: "100vw", height: "100vh", margin: 0}}>
				<Layout style={{width: "100%", height: "100%"}}>

					<Layout.Content>
						<Tabs
								style={{
									height: "100%",
									width: "100%",
								}}
								tabPosition={"left"}
								items={GenerateTabs(GameList, pdx_logo)}
						/>
					</Layout.Content>

				</Layout>

			</AntdApp>
	);
}

export default App;
