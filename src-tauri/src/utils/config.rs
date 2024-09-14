use serde::Deserialize;
use serde_json::Value;
use std::{fs, io, path::Path};

// 读取 JSON 配置文件，返回一个 json::Value
#[derive(Debug, Deserialize)]
pub struct Config {
    config: Value,
}
impl Default for Config {
    fn default() -> Self {
        Config {
            config: Value::Null,
        }
    }
}
#[allow(dead_code)]
impl Config {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        // 打开文件
        let file = fs::File::open(path).expect("file not found");
        // 创建一个缓冲读取器
        let reader = io::BufReader::new(file);
        // 解析JSON文件
        let config = serde_json::from_reader(reader).expect("error while reading json file");
        Config { config }
    }

    pub fn get_data(&self) -> &Value {
        &self.config
    }
}
