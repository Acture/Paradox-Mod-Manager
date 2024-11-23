use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "paradox.pest"] // 指向你的 Pest 语法文件
struct ConfigParser;

fn main() {

    let config_content = r#"
        version="0.0.1"
        dependencies={
            "Missions Expanded"
        }
        tags={
            "Utilities"
        }
        name="Infinite Mission Reward for Mission Expanded"
        supported_version="1.36.0"
        path="C:/Users/actur/Documents/Paradox Interactive/Europa Universalis IV/mod/infinite_mission_reward"
        remote_file_id="2636386736"
    "#;

    match ConfigParser::parse(Rule::input, config_content) {  // 修正为使用 `Rule::input`
        Ok(parsed) => {
            for record in parsed {
                println!("{:?}", record);
            }
        }
        Err(err) => eprintln!("Parsing error: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use crate::main;

    #[test]
    fn it_works() {
        main();
    }
}
