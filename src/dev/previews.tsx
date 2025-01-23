import {ComponentPreview, Previews} from "@react-buddy/ide-toolbox";
import {PaletteTree} from "./palette";
import App from "../App.tsx";
import GameConfig from "../components/GameConfig.tsx";

const ComponentPreviews = () => {
	return (
		<Previews palette={<PaletteTree/>}>
			<ComponentPreview path="/App">
				<App/>
			</ComponentPreview>
			<ComponentPreview path="/GameConfig">
				<GameConfig game_name={""} game_dir={null} mod_dir={null}/>
			</ComponentPreview>
			<ComponentPreview path="/ComponentPreviews">
				<ComponentPreviews/>
			</ComponentPreview>
		</Previews>
	);
};

export default ComponentPreviews;