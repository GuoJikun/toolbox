use std::net::{SocketAddr, TcpListener};

use std::sync::Mutex;
use tauri::State;

// 定义共享状态，用于控制服务器的启动和停止
#[derive(Default, Debug)]
pub struct ServerStateInner {
    pub is_running: bool,
}

pub type ServerState = Mutex<ServerStateInner>;

pub fn get_local_ip() -> String {
    match local_ip_address::local_ip() {
        Ok(local_ip_address) => local_ip_address.to_string(),
        Err(_) => "0.0.0.0".to_string(),
    }
}

fn has_available_port(ip: &str, port: u16) -> bool {
    let addr: SocketAddr = format!("{}:{}", ip, port).parse().unwrap();
    TcpListener::bind(addr).is_ok()
}

pub fn start_file_server(
    state: State<'_, ServerState>,
    shared_dir: &str,
) -> Result<String, String> {
    // 检查是否已经有服务器在运行
    let mut state = state.lock().unwrap();

    if state.is_running {
        return Err("Server is already running!".to_string());
    }

    // let shared_dir = "./shared_files";
    let ip = get_local_ip(); // 监听本地 IP

    let port = 8863;

    // 查找可用端口
    let available_port = has_available_port(&ip, port);
    if available_port {
        let file_server = warp::fs::dir(shared_dir.to_string());
        let addr = format!("{}:{}", &ip, port).parse::<SocketAddr>().unwrap();
        tokio::spawn(async move {
            warp::serve(file_server).run(addr).await;
        });
        state.is_running = true; // 标记服务器已启动
        return Ok(format!("File server running at http://{}:{}/", ip, port));
    } else {
        return Err("No available port found in the range 8063-8067.".to_string());
    }
}

pub fn stop_file_server(state: State<'_, ServerState>) -> Result<String, String> {
    let mut state = state.lock().unwrap();

    if !state.is_running {
        return Err("No server is running.".to_string());
    }

    // 停止逻辑（此处仅示例，需进一步实现停止 warp 服务器的方法）
    state.is_running = false;

    Ok("File server stopped.".to_string())
}
