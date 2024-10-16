import type { PluginConfig } from "@/utils/typescript"

export const internalPlugins: PluginConfig[] =[
    {
        type: 'module',
        id: 'colorConversion',
        name: '颜色转换',
        main: '/plugins/color-conversion',
        keywords: [
            'conversion',
            'color',
            'color-conversion',
            'color-convert',
            '颜色转换'
        ],
        description: '颜色值转换插件',
        version: '0.1.0',
        permissions: [],
        windowConfig: {},
    }
]

export default internalPlugins