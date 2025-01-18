import React from "react";
import {Card} from "antd";

interface GameTabProps {
	game_name: string;
	img_src: string;
}

const GameTab: React.FC<GameTabProps> = ({game_name, img_src}) => {
	return (
			<Card
					size="small"
					hoverable
					bordered
					cover={
						<img
								alt={game_name}
								src={img_src}
								style={{
									maxWidth: "200px",
									objectFit: "cover",
								}}
						/>
					}
			>
				<Card.Meta title={game_name}/>
			</Card>
	);
};

export default GameTab;
