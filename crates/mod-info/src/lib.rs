pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub struct ModInfo {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<String>,
    pub tags: Vec<String>,
    pub supported_version: String,
    pub local_path: String,
    pub remote_file_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
