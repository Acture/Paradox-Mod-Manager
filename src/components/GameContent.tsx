import React from "react";
import { Layout, Row, Col} from "antd";
import GameConfig from "./GameConfig.tsx";

interface GameContentProps {
	game_name: string;
	game_dir: string | null;
	mod_dir: string | null;
	game_logo: string | null;
}


const GameContent: React.FC<GameContentProps> = ({game_name, game_dir, mod_dir, game_logo}) => {


	return (
			<Layout>
				<Layout.Header style={{backgroundColor: "transparent"}}/>
				<Layout.Content style={{width:"100%"}}>
					<Row justify={"space-around"} align={"middle"} style={{width:"100%"}}>
						<Col span={11}>
							<div style={{maxWidth:"100%"}}>
								<img
										alt={game_name}
										src={game_logo ? game_logo : ""}
										style={{
											width: "100%",     // 确保图片宽度不超过 div 宽度
											height: "auto",
											objectFit: "cover",
										}}
								/>
							</div>
						</Col>
						<Col span={11}>
							<GameConfig game_name={game_name} game_dir={game_dir} mod_dir={mod_dir}/>

						</Col>
					</Row>


				</Layout.Content>
				<Layout.Footer>
					<div></div>
				</Layout.Footer>
			</Layout>
	);
};

export default GameContent;
