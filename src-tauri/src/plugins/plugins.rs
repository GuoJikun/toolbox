use md5;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::process::{Command, Output};
use tauri::command;

#[command]
pub fn run_node_script(script: String, args: Vec<String>) -> Result<String, String> {
    let mut full_args = vec![script];
    full_args.extend(args);

    let output: Output = Command::new("node")
        .args(&full_args)
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Err(stderr)
    }
}

#[command]
pub fn run_php_script(script: String, args: Vec<String>) -> Result<String, String> {
    run_script("php", script, args)
}

#[command]
pub fn run_python_script(script: String, args: Vec<String>) -> Result<String, String> {
    run_script("python", script, args)
}

fn run_script(command: &str, script: String, args: Vec<String>) -> Result<String, String> {
    let mut full_args = vec!["-c".to_string(), script];
    full_args.extend(args);
    println!("Running script: {} {:?}", command, full_args);
    let output: Output = Command::new(command)
        .args(&full_args)
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Err(stderr)
    }
}

const HEADER: &[u8; 4] = b"PLUG"; // 自定义文件头
pub struct Tools;

impl Tools {
    pub fn create(plugin_data_path: &str, plugin_package_path: &str) -> io::Result<()> {
        // 读取插件数据
        let mut data_file = File::open(plugin_data_path)?;
        let mut data = Vec::new();
        data_file.read_to_end(&mut data)?;

        // 计算插件数据的MD5校验和
        let hasher = md5::compute(&data);

        let checksum_str = format!("{:x}", hasher);

        // 创建插件包文件
        let mut package_file = File::create(plugin_package_path)?;

        // 写入文件头
        package_file.write_all(HEADER)?;

        // 写入校验和
        package_file.write_all(checksum_str.as_bytes())?;

        // 写入数据大小
        let data_size = (data.len() as u32).to_le_bytes();
        package_file.write_all(&data_size)?;

        // 写入插件数据
        package_file.write_all(&data)?;

        println!("Plugin package created successfully");
        Ok(())
    }

    pub fn verify(plugin_path: &str) -> io::Result<bool> {
        let mut file = File::open(plugin_path)?;

        // 读取并验证文件头
        let mut header = [0u8; 4];
        file.read_exact(&mut header)?;
        if &header != HEADER {
            eprintln!("Invalid file header");
            return Ok(false);
        }

        // 读取存储的校验和
        let mut stored_checksum = [0u8; 32];
        file.read_exact(&mut stored_checksum)?;

        // 读取插件数据大小
        let mut size_buf = [0u8; 4];
        file.read_exact(&mut size_buf)?;
        let data_size = u32::from_le_bytes(size_buf) as usize;

        // 读取插件数据
        let mut data = vec![0u8; data_size];
        file.read_exact(&mut data)?;

        // 计算插件数据的MD5校验和
        let hasher = md5::compute(&data);

        let calculated_checksum = format!("{:x}", hasher);

        // 比较计算的校验和与存储的校验和
        Ok(&calculated_checksum.as_bytes() == &stored_checksum)
    }

    pub fn install(plugin_path: &str, install_dir: &str) -> io::Result<()> {
        let mut file = File::open(plugin_path)?;

        // 跳过文件头和校验和
        file.seek(SeekFrom::Start(4 + 32))?;

        // 读取插件数据大小
        let mut size_buf = [0u8; 4];
        file.read_exact(&mut size_buf)?;
        let data_size = u32::from_le_bytes(size_buf) as usize;

        // 读取插件数据
        let mut data = vec![0u8; data_size];
        file.read_exact(&mut data)?;

        // 创建安装目录
        let install_path = Path::new(install_dir);
        std::fs::create_dir_all(install_path)?;

        // 保存插件数据到文件
        let plugin_file_path = install_path.join("plugin.bin");
        let mut plugin_file = File::create(plugin_file_path)?;
        plugin_file.write_all(&data)?;

        println!("Plugin installed successfully");
        Ok(())
    }

    pub fn verify_and_install(plugin_path: &str, install_dir: &str) -> io::Result<()> {
        if !Tools::verify(plugin_path)? {
            eprintln!("Checksum verification failed");
            return Ok(());
        }

        Tools::install(plugin_path, install_dir)?;
        Ok(())
    }
}
