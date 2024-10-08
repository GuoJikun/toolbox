# Toolbox

使用 tauri 开发的一个工具集软件

## 插件设想

```md
plguin-A
    - config.json
    - index.html
```

```json
// plugin/config.json
{
    "id": "", // 插件的唯一标识，为了防止重复最好加上自己的名字
    "name": "", // 插件的名字
    "main": "plugin-a/index.html", // 默认是插件目录下的 index.html(要包含插件目录)
    "primissions": [], // 同 tauri 的权限
    "keywords": [], // 在软件中搜索的关键字
    "description": "", // 插件描述
    "version": "",
    "author": "",
    "email": "",
    "homeUrl": "https://xxxx.com",
    "type": "backend/frontend"
}
```

## 设置项

- 基础设置
  - 唤起搜索页面的快捷键
- 插件设置
  - 插件目录
