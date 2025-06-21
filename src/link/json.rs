use std::fs;
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub cache: Vec<String>,
    pub source: String,
}

/// 读取 JSON 文件并返回解析后的结构体
pub fn read_config(file_path: &str) -> io::Result<Config> {
    let data = fs::read_to_string(file_path)?;
    let config: Config = serde_json::from_str(&data)?;
    Ok(config)
}

/// 将配置写入 JSON 文件
pub fn write_config(file_path: &str, config: &Config) -> io::Result<()> {
    let json_data = serde_json::to_string_pretty(config)?;
    fs::write(file_path, json_data)?;
    Ok(())
}

/// 向 `cache` 字段添加一个新值
pub fn add_to_cache(file_path: &str, value: String) -> io::Result<()> {
    let mut config = read_config(file_path)?;
    config.cache.push(value);
    write_config(file_path, &config)?;
    Ok(())
}

/// 从 `cache` 字段中删除指定值
pub fn remove_from_cache(file_path: &str, value: String) -> io::Result<()> {
    let mut config = read_config(file_path)?;
    config.cache.retain(|item| item != &value);
    write_config(file_path, &config)?;
    Ok(())
}

/// 修改 `source` 字段的值
pub fn update_source(file_path: &str, new_source: String) -> io::Result<()> {
    let mut config = read_config(file_path)?;
    config.source = new_source;
    write_config(file_path, &config)?;
    Ok(())
}