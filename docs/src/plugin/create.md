# 插件开发

## 独立的可执行程序

在插件的 `config.json` 中 `type` 字段对应的是 `binary`

可以使用任意语言开发的可独立执行的程序，不同的平台需要不同的格式（windows：exe 等）。

## 脚本

在插件的 `config.json` 中 `type` 字段对应的是 `script`

目前支持的脚本类型：`node`、`php`、`python`，这类脚本就是对应程序的可执行文件，但是最后的执行结果需要**_输出到终端_**

## 独立窗口类

在插件的 `config.json` 中 `type` 字段对应的是 `module`，入口是一个 html 类型的文件，在配置的文件的 `main` 中声明

:::tip 窗口的默认权限

```json
{
    "permission": [
        "core:path:default", 
        "core:event:default", 
        "core:window:default", 
        "core:app:default", 
        "core:image:default", 
        "core:resources:default", 
        "core:menu:default", 
        "core:tray:default"
    ]
}
```

:::

## 插件配置

```json
{
    "type": "module",
    "id": "screenshot",
    "name": "截图",
    "main": "index.html",
    "devMain": "http://localhost:5173/",
    "keywords": ["screenshot", "截图"],
    "description": "screenshot",
    "version": "0.1.0",
    "primission": ["core:default"],
    "windowConfig": {
        "fullscreen": true,
        "alwaysOnTop": true,
        "skipTaskbar": false,
        "focus": true,
        "decorations": false,
        "resizable": false,
        "transparent": true
    }
}
```

### type

用来注明插件的类型，分为三类，分别是 `binary`、`script`、`module`

### id
