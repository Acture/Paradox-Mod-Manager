use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use std::string::String;

#[derive(Parser)]
#[grammar = "paradox.pest"] // 指向你的 Pest 语法文件
struct ParadoxParser;

#[derive(Debug, Clone)]
enum DictValue {
	Int(i64),
	Float(f64),
	String(String),
	Object(HashMap<DictKey, DictValue>),
	Array(Vec<DictValue>),
	Bool(bool),
	Null,
}


#[derive(Debug, Clone, Eq, Hash, PartialEq)]
enum DictKey {
	String(String),
	Int(i64),
}

impl ParadoxParser {
	fn parse_value(token: Pair<Rule>) -> Result<DictValue, Error<Rule>> {
		match token.as_rule() {
			Rule::NUMBER => {
				Ok(Self::parse_value(token.into_inner().next().unwrap())?)
			}
			Rule::INTEGER => {
				let number = token.as_str().parse::<i64>().unwrap();
				Ok(DictValue::Int(number))
			}
			Rule::FLOAT => {
				let number = token.as_str().parse::<f64>().unwrap();
				Ok(DictValue::Float(number))
			}
			Rule::value => {
				let inner = token.into_inner().next().unwrap();
				Ok(Self::parse_value(inner)?)
			}
			Rule::quoted_word => {
				Ok(DictValue::String(token.as_str().to_string()))
			}
			Rule::array_list => {
				let mut arr = Vec::new();
				for pair in token.into_inner() {
					arr.push(Self::parse_value(pair)?);
				}
				Ok(DictValue::Array(arr))
			}
			Rule::json_object => {
				let mut map = HashMap::new();
				for pair in token.into_inner() {
					let mut inner_rules = pair.into_inner();
					let key = inner_rules.next().unwrap();
					let key = Self::parse_key(key).unwrap();
					let value = inner_rules.next().unwrap();
					let value = Self::parse_value(value).unwrap();
					if let Some(prev) = map.get_mut(&key) {
						if let DictValue::Array(array) = prev {
							array.push(value);
						} else {
							let new_array = vec![prev.clone(), value];
							map.insert(key.clone(), DictValue::Array(new_array));
						}
					} else {
						map.insert(key, value);
					}
				}
				Ok(DictValue::Object(map))
			}
			Rule::WORD => {
				let s = token.as_str();
				match s {
					"yes" => Ok(DictValue::Bool(true)),
					"no" => Ok(DictValue::Bool(false)),
					_ => Ok(DictValue::String(s.to_string())),
				}
			}
			_ => {
				unimplemented!("{:#?}", token.as_rule())
			}
		}
	}

	fn parse_key(token: Pair<Rule>) -> Result<DictKey, Error<Rule>> {
		match token.as_rule() {
			Rule::key => {
				let inner = token.into_inner().next().unwrap();
				Ok(Self::parse_key(inner)?)
			}
			Rule::WORD => {
				Ok(DictKey::String(token.as_str().to_string()))
			}
			Rule::quoted_word => {
				Ok(DictKey::String(token.as_str().to_string()))
			}
			Rule::INTEGER => {
				let number = token.as_str().parse::<i64>().unwrap();
				Ok(DictKey::Int(number))
			}
			_ => {
				unimplemented!("{:#?}", token.as_rule())
			}
		}
	}
	pub fn parse_to_json(input: &str) -> Result<HashMap<DictKey, DictValue>, Error<Rule>> {
		let input_pairs = Self::parse(Rule::input, input).unwrap().next().unwrap();
		let mut result = HashMap::<DictKey, DictValue>::new();
		for pair in input_pairs.into_inner() {
			match pair.as_rule() {
				Rule::pair => {
					let mut inner_rules = pair.into_inner();
					let key = inner_rules.next().unwrap();
					let key = Self::parse_key(key).unwrap();
					let value = inner_rules.next().unwrap();
					let value = Self::parse_value(value).unwrap();

					result.insert(key, value);
				}
				Rule::EOI => continue,
				_ => unreachable!("{:#?}", pair.as_rule()),
			}
		}
		Ok(result)
	}
}
impl std::fmt::Display for DictKey {
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			DictKey::String(s) => write!(formatter, "{}", s),
			DictKey::Int(i) => write!(formatter, "{}", i),
		}
	}
}
impl std::fmt::Display for DictValue {
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			DictValue::Null => write!(formatter, "null"),
			DictValue::Bool(b) => write!(formatter, "{}", b),
			DictValue::Float(f) => write!(formatter, "{}", f),
			DictValue::Int(i) => write!(formatter, "{}", i),
			DictValue::String(s) => write!(formatter, "\"{}\"", s),
			DictValue::Array(arr) => {
				write!(formatter, "[")?;
				for (i, value) in arr.iter().enumerate() {
					if i > 0 {
						write!(formatter, ", ")?
					}
					write!(formatter, "{}", value)?;
				}
				write!(formatter, "]")
			}
			DictValue::Object(map) => {
				write!(formatter, "{{")?;
				for (i, (key, value)) in map.iter().enumerate() {
					if i > 0 {
						write!(formatter, ", ")?
					}
					write!(formatter, "\"{}\": {}", key, value)?;
				}
				write!(formatter, "}}")
			}
		}
	}
}

struct DisplayHashMap<'a>(pub &'a HashMap<DictKey, DictValue>);
impl<'a> std::fmt::Display for DisplayHashMap<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{{\n")?;
		for (i, (key, value)) in self.0.iter().enumerate() {
			write!(f, "\t{}: {}", key, value)?;
			if i > 0 {
				write!(f, ", ")?;
			}
			write!(f, "\n")?;
		}
		write!(f, "}}")
	}
}
fn main() {}

#[cfg(test)]
mod tests {
	use crate::{DisplayHashMap, ParadoxParser};
	use std::fs;

	#[test]
	fn parse_mod() {
		let input = r#"
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
		let result = ParadoxParser::parse_to_json(input).unwrap();
		println!("{}", DisplayHashMap(&result));
	}

	#[test]
	fn parse_idea() {
		let content = fs::read_to_string("data/00_basic_ideas.txt")
			.unwrap_or_else(|error| panic!("读取文件内容失败：{:?}", error));
		let result = ParadoxParser::parse_to_json(&content).unwrap();
		println!("{}", DisplayHashMap(&result));
	}
}
