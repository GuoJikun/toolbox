use serde::Deserialize;
use serde_json;
use std::fs;
use std::io;
use std::path::Path;
use walkdir::WalkDir;

pub fn copy_dir_all(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in WalkDir::new(src).min_depth(1) {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path
            .strip_prefix(src)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
        let destination = dst.join(relative_path);

        if path.is_dir() {
            fs::create_dir_all(&destination)?;
        } else if path.is_file() {
            fs::copy(&path, &destination)?;
        }
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct Config {
    plugin_root: String,
}
pub fn read_json_file<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn std::error::Error>> {
    // 打开文件
    let file = fs::File::open(path)?;
    // 创建一个缓冲读取器
    let reader = io::BufReader::new(file);
    // 解析JSON文件
    let config = serde_json::from_reader(reader)?;
    Ok(config)
}
