import type { PluginConfig } from '@/utils/typescript'

export const internalPlugins: PluginConfig[] = [
    {
        type: 'module',
        id: 'colorConversion',
        name: '颜色转换',
        main: '/plugins/color-conversion',
        keywords: ['conversion', 'color', 'color-conversion', 'color-convert', '颜色转换'],
        description: '颜色值转换插件',
        version: '0.1.0',
        permissions: [],
        windowConfig: {}
    },
    {
        type: 'module',
        id: 'localServer',
        name: '本地服务',
        main: '/plugins/local-server',
        keywords: ['server', 'shared', 'file', '文件共享', '共享', '本地服务', 'local'],
        description: '提供一个本地服务，可以用于局域网传文件',
        version: '0.1.0',
        permissions: [],
        windowConfig: {}
    },
    {
        type: 'module',
        id: 'pkgManager',
        name: '软件管理',
        main: '/plugins/pkg-manager',
        keywords: ['软件', '升级', '安装', 'pkg', 'upgrade', 'update', 'install', '软件管理'],
        description: '使用 Winget 管理本机软件',
        version: '0.1.0',
        permissions: [],
        windowConfig: {
            width: 1000,
            height: 660,
            resizable: true
        }
    },
    {
        type: 'module',
        id: 'convertToWebp',
        name: '图片转Webp',
        main: '/plugins/convert-image-to-webp',
        keywords: ['图片', 'image', 'webp', 'convert', '格式', 'format', 'jpg', 'png'],
        description: '将图片转为 webp 格式',
        version: '0.1.0',
        permissions: ["log:allow-log", "fs:write-all", "fs:allow-copy-file"],
        windowConfig: {}
    }
]

export default internalPlugins
