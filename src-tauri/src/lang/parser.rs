use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{alphanumeric1, char, multispace0, multispace1, newline, space0};
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::{delimited, separated_pair};
use nom::IResult;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum ParsedValue {
	String(String),                     // 单独的字符串值
	MAP(HashMap<String, ParsedValue>), // 嵌套的 JSON 键值对
	List(Vec<ParsedValue>),             // 列表
}

impl ParsedValue {
	pub fn as_string(&self) -> Option<String> {
		match self {
			ParsedValue::String(s) => Some(s.to_string()),
			_ => None,
		}
	}

	pub fn as_array(&self) -> Option<&Vec<ParsedValue>> {
		match self {
			ParsedValue::List(arr) => Some(arr),
			_ => None,
		}
	}

	pub fn as_str(&self) -> Option<&str> {
		match self {
			ParsedValue::String(s) => Some(s),
			_ => None,
		}
	}

	pub fn as_map(&self) -> Option<&HashMap<String, ParsedValue>> {
		match self {
			ParsedValue::MAP(map) => Some(map),
			_ => None,
		}
	}
}

fn key(input: &str) -> IResult<&str, String> {
	alt((
		map(is_not(" ="), |res: &str| res.to_string()),
		map(quoted_string, |res: &str| res.to_string()),
	))(input)
}

fn value(input: &str) -> IResult<&str, ParsedValue> {
	alt((
		map(alphanumeric1, |res: &str| {
			ParsedValue::String(res.to_string())
		}),
		map(quoted_string, |res: &str| {
			ParsedValue::String(res.to_string())
		}),
		map(list, |res| ParsedValue::List(res)),
		map(json, |res| ParsedValue::MAP(res)),
	))(input)
}

fn kv_pair(input: &str) -> IResult<&str, (String, ParsedValue)> {
	separated_pair(key, delimited(multispace0, char('='), multispace0), value)(input)
}

fn json(input: &str) -> IResult<&str, HashMap<String, ParsedValue>> {
	map(
		delimited(char('{'), separated_list0(tag(","), kv_pair), char('}')),
		|pairs| HashMap::from_iter(pairs.into_iter()),
	)(input)
}

fn quoted_string(input: &str) -> IResult<&str, &str> {
	delimited(char('"'), is_not("\""), char('"'))(input)
}

fn list(input: &str) -> IResult<&str, Vec<ParsedValue>> {
	delimited(
		char('{'),
		delimited(
			multispace0,
			separated_list0(
				alt((
					delimited(multispace0, tag(","), multispace0), // 支持逗号分隔
					multispace1,                                   // 支持空格分隔
				)),
				value,
			),
			multispace0,
		),
		char('}'),
	)(input)
}

fn top_level(input: &str) -> IResult<&str, HashMap<String, ParsedValue>> {
	map(
		separated_list0(
			newline, // 必须是换行符来分隔每一行
			delimited(space0, kv_pair, space0),
		),
		|pairs| HashMap::from_iter(pairs.into_iter()), // 转换为 JSON
	)(input)
}

pub fn parse_content(content: &str) -> Result<HashMap<String, ParsedValue>, String> {
	match top_level(content) {
		Ok((_, json)) => Ok(json),
		Err(e) => Err(format!("Failed to parse content: {:?}", e)),
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_key_value_pairs() {
		let input = "key1=value1";
		let result = kv_pair(input);
		match result {
			Ok((rest, (key, value))) => {
				println!("rest: {}, key: {}, value: {:?}", rest, key, value);
				assert_eq!(rest, "");
				assert_eq!(key, "key1");
				if let ParsedValue::String(inner_value) = value {
					assert_eq!(inner_value, "value1");
				} else {
					panic!("Expected ParsedValue::String but got {:?}", value);
				}
			}
			Err(_) => panic!("parse kv_pair failed"),
		}
	}

	#[test]
	fn test_parse_kv_list() {
		let input = "tags={\n\t\"Utilities\"\n}";
		let result = kv_pair(input);
		match result {
			Ok((rest, (key, value))) => {
				println!("rest: {}, key: {}, value: {:?}", rest, key, value);
				assert_eq!(rest, "");
				assert_eq!(key, "tags");
				if let ParsedValue::List(inner_value) = value {
					assert_eq!(inner_value.len(), 1);
				} else {
					panic!("Expected ParsedValue::List but got {:?}", value);
				}
			}
			Err(e) => panic!("parse kv_pair failed: {:?}", e),
		}
	}

	#[test]
	fn test_parse_quoted_string() {
		let input = "\"value1\"";
		let result = quoted_string(input);
		match result {
			Ok((rest, value)) => {
				println!("rest: {}, value: {:?}", rest, value);
				assert_eq!(rest, "");
				assert_eq!(value, "value1");
			}
			Err(_) => panic!("parse quoted_string failed"),
		}
	}

	#[test]
	fn test_parse_json() {
		let input = "{key1=\"value1\",\"key2\"=\"value2\"}";
		let result = json(input);
		match result {
			Ok((rest, json)) => {
				println!("rest: {}, json: {:?}", rest, json);
				assert_eq!(rest, "");
				assert_eq!(
					json.get("key1"),
					Some(&ParsedValue::String("value1".to_string()))
				);
				assert_eq!(
					json.get("key2"),
					Some(&ParsedValue::String("value2".to_string()))
				);
			}
			Err(_) => panic!("parse json failed"),
		}
	}

	#[test]
	fn test_parse_list() {
		let input_1 = "{\"value1\" \"value2\"}";
		let result = list(input_1);
		match result {
			Ok((rest, list)) => {
				println!("rest: {}, list: {:?}", rest, list);
				assert_eq!(rest, "");
				assert_eq!(
					list.get(0),
					Some(&ParsedValue::String("value1".to_string()))
				);
				assert_eq!(
					list.get(1),
					Some(&ParsedValue::String("value2".to_string()))
				);
			}
			Err(e) => panic!("parse list failed: {:?}", e),
		}

		let input_2 = "{value1, value2}";
		let result = list(input_2);
		match result {
			Ok((rest, list)) => {
				println!("rest: {}, list: {:?}", rest, list);
				assert_eq!(rest, "");
				assert_eq!(
					list.get(0),
					Some(&ParsedValue::String("value1".to_string()))
				);
				assert_eq!(
					list.get(1),
					Some(&ParsedValue::String("value2".to_string()))
				);
			}
			Err(e) => panic!("parse list failed: {:?}", e),
		}
	}

	#[test]
	fn test_toplevel() {
		let input = "version=\"0.0.1\"\nname=\"defines\"";
		let result = top_level(input);
		match result {
			Ok((rest, json)) => {
				println!("rest: {}, json: {:?}", rest, json);
				assert_eq!(rest, "");
				assert_eq!(
					json.get("version"),
					Some(&ParsedValue::String("0.0.1".to_string()))
				);
				assert_eq!(
					json.get("name"),
					Some(&ParsedValue::String("defines".to_string()))
				);
			}
			Err(e) => panic!("parse content failed: {:?}", e),
		}
	}

	#[test]
	fn test_file() {
		let content = include_str!("../../tests/resources/defines.mod");
		let result = top_level(content);
		match result {
			Ok((rest, json)) => {
				println!("rest: {}, json: {:?}", rest, json);
				assert_eq!(rest, "");
				assert_eq!(json.len(), 6);
				assert_eq!(
					json.get("version"),
					Some(&ParsedValue::String("0.0.1".to_string()))
				);
				assert_eq!(
					json.get("name"),
					Some(&ParsedValue::String("defines".to_string()))
				);
				assert_eq!(
					json.get("tags"),
					Some(&ParsedValue::List(vec![ParsedValue::String(
						"Utilities".to_string()
					)]))
				);
				assert_eq!(
					json.get("supported_version"),
					Some(&ParsedValue::String("1.34.5".to_string()))
				)
			}
			Err(e) => panic!("parse content failed: {:?}", e),
		}
	}
}
