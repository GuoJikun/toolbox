import { defineConfig } from 'vitepress'

const sidebar = [
    {
        text: '指南'
    },
    {
        text: '插件',
        items: [
            { text: '插件概述', link: '/plugin/index' },
            { text: '插件列表', link: '/plugin/list' },
            { text: '插件开发', link: '/plugin/create' }
        ]
    }
]

export default defineConfig({
    title: 'Toolbox',
    lang: 'zh-CN',
    lastUpdated: true,
    themeConfig: {
        nav: [{ text: '插件', link: '/plugin/' }],
        sidebar,
        outline: {
            level: [2, 3],
            label: '章节目录'
        },
        docFooter: {
            prev: '上一页',
            next: '下一页'
        },
        lastUpdatedText: '最后更新时间',
        search: {
            provider: 'local'
        },
        langMenuLabel: '多语言',
        returnToTopLabel: '回到顶部',
        sidebarMenuLabel: '菜单',
        darkModeSwitchLabel: '主题',
        lightModeSwitchTitle: '切换到浅色模式',
        darkModeSwitchTitle: '切换到深色模式'
    }
})
