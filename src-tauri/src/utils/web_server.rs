use rocket::catch;
use rocket::fs::{FileServer};
use rocket::http::{Status, ContentType};
use rocket::request::Request;
use rocket::response::status;
use rocket::config::{Config, Ident};
use tauri::{path::BaseDirectory, AppHandle, Manager};


/// 自定义错误捕获器 - 404 未找到
#[catch(404)]
fn not_found(req: &Request) -> (Status, (ContentType, String)) {
    log::warn!("路由未找到: {}", req.uri());
    
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <meta charset="utf-8">
        <title>404 - 页面未找到</title>
        <style>
            body { font-family: Arial, sans-serif; text-align: center; padding: 50px; }
            h1 { color: #333; }
            .error-code { font-size: 72px; color: #e74c3c; }
        </style>
    </head>
    <body>
        <div class="error-code">404</div>
        <h1>页面未找到</h1>
        <p>您请求的页面 <code>#{uri}</code> 不存在。</p>
        <p><a href="/">返回首页</a></p>
    </body>
    </html>
    "#.replace("#{uri}", &req.uri().to_string());

    (Status::NotFound, (ContentType::HTML, html))
}

/// 自定义错误捕获器 - 500 服务器错误
#[catch(500)]
fn server_error() -> status::Custom<&'static str> {
    log::error!("服务器内部错误");
    status::Custom(Status::InternalServerError, "服务器发生了内部错误")
}

// Rocket 服务器实现
pub async fn init(app: AppHandle) -> Result<(), rocket::Error> {
    log::info!("初始化Rocket静态文件服务器...");
    log::info!("启动静态文件服务器 - http://127.0.0.1:6543");

    let binding = app
        .path()
        .resolve("plugins", BaseDirectory::Resource)
        .unwrap();

    let static_path = binding.to_str().unwrap();
    
    // 创建自定义配置，设置端口为 54321
    let config = Config::figment()
        .merge(("port", 54321))
        .merge(("address", "127.0.0.1"))
        .merge(("ident", Ident::none()));
    
    let _ = rocket::custom(config)
        .mount("/", FileServer::from(static_path))
        .register("/", rocket::catchers![not_found, server_error])
        .launch()
        .await?;
    
    Ok(())
}
