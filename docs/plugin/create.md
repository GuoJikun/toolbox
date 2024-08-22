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
    "author": "string",
    "email": "string",
    "homepage": "string",
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

插件的唯一标识，全局唯一，只支持英文、数字、_、-且不能数字开头

### name

插件显示的名称（可以重复）

### main

插件的入口点，路径是相对于自己的插件目录的。

### keywords

关键词，在搜索界面搜索用

### description

插件描述

### version

插件版本

### author

插件作者

### email

插件作者的邮箱

### homepage

插件的地址

### primission

插件的权限，用来生成 [`tauri`](https://beta.tauri.app/start/) 中 [`capabilities`](<https://beta.tauri.app/security/capabilities/>) 文件

### windowConfig

:::warning 注意
仅对 `type: "moudle"` 类型的插件有效
:::

窗口配置，具体内容请[参考](https://beta.tauri.app/reference/config/#windowconfig)
